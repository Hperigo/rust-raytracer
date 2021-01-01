extern crate image;

use std::sync::*;
use std::time::Instant;

mod color;
mod vec;
mod ray;
mod geometry;
mod renderer;
mod materials;
use renderer::{RenderData, Tile};

mod camera;
use camera::Camera;


fn main() {
    
    let render_data  = std::sync::Arc::new( RwLock::new( RenderData::new() ) ); 

    let start = Instant::now();
    let w = 500;
    let h = 250;
    
    {
        let mut render_data_w = render_data.write().unwrap();
        render_data_w.render_width = w;
        render_data_w.render_height = h;
        render_data_w.render_aspect_ratio = w as f32 / h as f32; 
        render_data_w.camera = Camera::new(render_data_w.render_aspect_ratio);
    }

    let num_of_tiles = 4;
    let single_tile_width = w/num_of_tiles;
    let single_tile_height = h/num_of_tiles;
 
    let mut tiles : Vec<Tile> = Vec::new();
    let mut data : Vec<u8> = Vec::new();
    data.resize(w * h * 3, 0);

    for x in 0.. ((w + single_tile_width) / single_tile_width){
        for y in 0.. ((h + single_tile_height) / single_tile_height){

            // calculate x and y position from each tile
            let tile_x_pos = x * single_tile_width;
            let tile_y_pos = y * single_tile_height;
            let mut tile_width = single_tile_width;
            let mut tile_height = single_tile_height;

            // if the tile size does not match the size of the image, we need to write a tile with "left over"
            if tile_x_pos + single_tile_width > w {
                let delta = (tile_x_pos + single_tile_width) - w;
                tile_width = single_tile_width - delta;
            }

            if tile_y_pos + single_tile_height > h {
                let delta = (tile_y_pos + single_tile_height) - h;
                tile_height = single_tile_height - delta;
            }

            if tile_width == 0 || tile_height == 0 {
                continue;
            }

            let tile = Tile::new(

                             tile_x_pos, 
                             tile_y_pos,
                             tile_width,
                             tile_height,
                        );
            tiles.push(tile);
        }
    }

    println!("number of tiles: {}, tile height: {}", tiles.len(), single_tile_height);

    
    let num_of_threads = {
        let num = 4;
        if tiles.len() < num {
            tiles.len()
        }else{
            num
        }
    };

    let part = tiles.len() / num_of_threads;
    let mut handles = Vec::with_capacity(num_of_threads);
    
    println!("Rendering with {} threads", num_of_threads);

    for _ in 0..num_of_threads {

        //split the work per thread
        let mut thread_tiles = Vec::with_capacity(part);
        for _ in 0..part {
            thread_tiles.push( tiles.pop().unwrap() );
        }

        let w = render_data.clone();
        let j = std::thread::spawn(move || {
            let mut i = 0; 
            for t in  &mut thread_tiles{
                t.run(&w);
                i = i + 1;
            }
            return thread_tiles;
        });

        handles.push(j);
    }

    for k in handles {
       let rendered_tiles =  k.join().unwrap();
       for t in rendered_tiles{
            t.write_data(&mut data, w, h);
        }
    }

    let render_duration = start.elapsed().as_millis();
    println!("1- program took: {}ms", render_duration);

    let mut img : image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::from_raw(w as u32, h as u32, data).ok_or("Error Creating buffer").unwrap();
    img = image::imageops::resize(&img, ((w as f32) * 1.) as u32, ((h as f32) * 1.0) as u32, image::imageops::FilterType::Lanczos3);
    img = image::imageops::flip_vertical(&img);
    img.save("test.png").unwrap();
    
    println!("2- writing file to disk took: {}ms", start.elapsed().as_millis() - render_duration);
}
