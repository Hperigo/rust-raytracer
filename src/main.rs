extern crate image;

use std::sync::*;
use std::time::Instant;

mod color;
mod vec;
mod ray;
mod geometry;
mod renderer;
mod materials;
mod hitrecord;
use renderer::{RenderData, Tile};

use vec::Vec3;
mod camera;
use camera::Camera;


use crate::materials::{Lambertian, Metal, Dieletric};
use crate::geometry::Sphere;
use crate::hitrecord::Hittable;

use rand::Rng;

fn create_random_scene() -> Vec<Box<dyn Hittable + Send + Sync>> {
    let mut objects : Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();

    // floor
    let ground_material = Box::new( Lambertian{ albedo :  Vec3::new(0.5, 0.5, 0.5) } );
    objects.push( Box::new( Sphere::new( Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            
            let chose_mat = rng.gen::<f32>();

            let center = Vec3::new( (a as f32) + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>() );

            if (center - Vec3::new(4.0, 0.2, 0.0) ).length() > 0.9 {

                if chose_mat < 0.8 {

                    let color = Vec3::random() * Vec3::random();

                    let sphere = Sphere::new( 
                        center,
                        0.2, 
                        Box::new( Lambertian{ albedo : color } ));

                    objects.push( Box::new(sphere) );  
                }
                else if chose_mat < 0.95 {
                    
                    let color = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere = Sphere::new( 
                        center,
                        0.2, 
                        Box::new( Metal{ 
                                            albedo : color,
                                            fuzz }
                                ));

                    objects.push( Box::new(sphere) );  
                }
                else
                {
                    let color = Vec3::random() * Vec3::random();

                    let sphere = Sphere::new( 
                        center,
                        0.2, 
                        Box::new( Dieletric{ ir : 1.5 } ));

                    objects.push( Box::new(sphere) );
                }
            }
        }




    }

    objects.push( Box::new( Sphere::new( Vec3::new(0.0, 1.0, 0.0),1.0, Box::new( Dieletric{ ir : 1.5 } ) )));      
    objects.push( Box::new( Sphere::new( Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new( Lambertian{ albedo : Vec3::new( 0.4, 0.2, 0.1 ) })   )));  
    objects.push( Box::new( Sphere::new( Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new( Metal{ albedo :  Vec3::new(0.7, 0.6, 0.5), fuzz : 0.0 } ) )));  

    objects
}

fn create_debug_scene() -> Vec<Box<dyn Hittable + Send + Sync>> {
    let mut objects : Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();

    let floor_material = Box::new( Lambertian{ albedo :  Vec3::new(0.9, 0.9, 0.1) } );
    // floor 
    objects.push( Box::new( Sphere::new( Vec3::new(0.0, -100.5, -1.0), 100.0, floor_material.clone()  )));

    objects.push( Box::new( Sphere::new( Vec3::new(-1.0, 0.0, -1.0),0.5, Box::new( Dieletric{ ir : 0.9 } ) )));  
    objects.push( Box::new( Sphere::new( Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new( Lambertian{ albedo :  Vec3::new(0.4, 0.4, 0.4) } ) )));  
    objects.push( Box::new( Sphere::new( Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new( Metal{ albedo :  Vec3::new(0.9, 0.8, 0.4), fuzz : 0.9 } ) )));  

    objects
}



fn main() {
    

    let start = Instant::now();
    let w = 1500;
    let h = 750;
  


    let camera = {

        let look_from = Vec3::new(7.0,5.4,-5.0);
        let look_at = Vec3::new(0.0,0.0,0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 5.0;
        
        Camera::new( 
            look_from,
            look_at,
            vup,
            20.0, 
            w  as f32 / h as f32,
            0.6,
            dist_to_focus,
        )
    };



    let render_data  = std::sync::Arc::new( 
        RwLock::new(
            RenderData::new( w, h, w as f32 / h as f32, 10, 50, camera, create_random_scene() )
    )); 

    let num_of_tiles = 6;
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
    
    println!("Rendering with {} threads, groups: {}", num_of_threads, part);

    for _ in 0..num_of_threads {

        //split the work per thread, theres a bug where not all tiles get passed throught the tiles.. need to figure out a way to add the left over from the division into the tile jobs.
        let mut thread_tiles = Vec::with_capacity(part);
        for _ in 0..part {
            thread_tiles.push( tiles.pop().unwrap() );
            println!("len: {}", tiles.len()); 
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
