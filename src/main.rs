extern crate image;
extern crate crossbeam;
extern crate rand;

use std::sync::*;
use std::time::Instant;

mod color;
use color::Color;

mod vec;
use vec::Vec3;

mod ray;
use ray::Ray;

mod camera;
use camera::Camera;

use rand::Rng;


#[derive(Clone)]
struct HitRecord{
    p : Vec3,
    normal : Vec3,
    t : f32,
    front_face : bool,
}


impl HitRecord{
    pub fn new()->Self{
        HitRecord{
            p : Vec3::zero(),
            normal : Vec3::zero(),
            t : 0.0,
            front_face : true,
        }
    }
    pub fn set_face_normal(&mut self, r : &Ray, outward_normal : &Vec3){
        self.front_face = Vec3::dot(&r.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face  { *outward_normal } else { *outward_normal * -1.0 }
    }
} 

trait Hittable{
    fn hit(&self, ray : &Ray, t_min : f32, t_max : f32, hit_record : &mut HitRecord ) -> bool;
}


struct Sphere {
    center : Vec3,
    radius : f32,
}

impl Sphere{ 
    
    pub fn new(center : Vec3, radius : f32) -> Self{
        Sphere{
            center,
            radius,
        }
    }
}


impl Hittable for Sphere {
    
    fn hit(&self, r : &Ray, t_min : f32, t_max : f32, hit_record : &mut HitRecord ) -> bool{
        let oc = r.origin - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(&oc, &r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0
        { 
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        let normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, &normal);
        return true;
    }
}


struct HittableList {
        objects : Vec<Box<dyn Hittable + Send + Sync>>,
}

impl Hittable for HittableList{
    
    
    fn hit(&self, r : &Ray, t_min : f32, t_max : f32, hit_record : &mut HitRecord ) -> bool{
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;

        let mut closest_so_far = t_max;

        for obj in &self.objects{
            if obj.hit(&r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec.clone();
            }
        }
        return hit_anything;
    }
}

impl HittableList{
    pub fn new() -> Self{
       let mut table = HittableList{
            objects : Vec::new(), 
       };
    
       table.objects.push( Box::new( Sphere::new( Vec3::new(0.0, 0.0, -1.0), 0.5 )));  
       table.objects.push( Box::new( Sphere::new( Vec3::new(0.0, -100.5, -1.0), 100.0 )));  

       return table;
    }
}

fn hit_sphere(center : &Vec3, radius : f32, r : &Ray ) -> f32 {
    
    let oc = r.origin - *center;
    let a = r.dir.length_squared();
    let half_b = Vec3::dot(&oc, &r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }else{
        return (- half_b - discriminant.sqrt() ) / a;
    }

}

fn ray_color(r : &Ray, hit_world : &HittableList) -> Vec3 {
    
    let mut hit_record = HitRecord::new();
    
    if hit_world.hit(r, 0.0, f32::INFINITY, &mut hit_record) {
        let cf = ( hit_record.normal + Vec3::one() ) * 0.5;
        // return Color::new_from_f32(cf.x, cf.y, cf.z);
        return cf;
    }

    let unit_vector = Vec3::normalize(r.dir);
    let t = 0.5 * (unit_vector.y + 1.0);
    let v =  Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t; 

    // Color::new_from_f32(v.x, v.y, v.z)
    v
}

struct RenderData {
    render_width : usize,
    render_height : usize,
    render_aspect_ratio : f32,

    hittable : HittableList,
    camera  : Camera,
}

impl RenderData{
    fn new() -> Self {
        RenderData{
            render_width : 0,
            render_height : 0,
            render_aspect_ratio : 0.,

            hittable : HittableList::new(),
            camera : Camera::new(1.0),
        }   
    }
}
type RenderDataHandle = RwLock<RenderData>;

#[derive(Debug)]
struct Tile{
    x : usize,
    y : usize,
    w : usize,
    h : usize,
    
    data : Vec<Color<u8>>,
}

impl Tile{
    pub fn new(x : usize, y : usize, w : usize, h : usize ) -> Self {
        let data : Vec<Color<u8>> = Vec::new();
        Tile{
            x,y,w,h,
            data
        }
    }

    fn world_location_of_pixel(&self, x : usize, y : usize ) -> (usize, usize) {
        let xx = self.x + x;
        let yy = self.y + y;
        (xx, yy)
    }

    pub fn run<'a>(&mut self, render_data : &'a RenderDataHandle){
        self.data.resize(self.w * self.h, Color::black() );
        //println!("Running on thread-id: {:?}", std::thread::current().id() ); 

        let world = render_data.read().unwrap();
        
        let viewport_height = 2.0;
        let viewport_width = viewport_height * world.render_aspect_ratio;
        let focal_length = 1.0;

        let origin = Vec3::zero();
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.); 

        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

        let mut rng = rand::thread_rng();

        for y in 0..self.h{
            for x in 0..self.w{
                let screen_pos = self.world_location_of_pixel(x, y);
                
                let mut pixel_sample = Vec3::zero();
                let num_of_samples = 16;
                for i in 0..num_of_samples{

                    let u = (screen_pos.0 as f32 + rng.gen::<f32>() ) / (world.render_width as f32 - 1.0); 
                    let v = (screen_pos.1 as f32 + rng.gen::<f32>() ) / (world.render_height as f32  - 1.0); 
                    
                    let r = world.camera.get_ray(u, v);
                    pixel_sample = pixel_sample + ray_color(&r, &world.hittable);
                }


                pixel_sample = pixel_sample * (1.0 / num_of_samples as f32);
                let index = x + y * self.w;
                self.data[index] = Color::new_from_f32(pixel_sample.x, pixel_sample.y, pixel_sample.z); //Color::new_from_f32(red, green, 0.0);
           }
        }
    }

    pub fn write_data(&self, target : &mut Vec<u8>, w : usize, _h : usize ){
       for y in 0..self.h{
           for x in 0..self.w{
               let index = x + y * self.w;
               let index_target = (x + self.x) * 3 + (y + self.y) * 3 * w;
               target[index_target] = self.data[index].r;
               target[index_target + 1] = self.data[index].g;
               target[index_target + 2] = self.data[index].b;
            }
        }
    }
}

fn main() {
    
    let render_data  = std::sync::Arc::new( RwLock::new( RenderData::new() ) ); 

    let start = Instant::now();
    let w = 1000;
    let h = 500;
    
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
            println!("tile {:?}", tile);
            tiles.push(tile);
        }
    }

    println!("number of tiles: {}, tile height: {}", tiles.len(), single_tile_height);

    
    let num_of_threads = {
        let num = 8;
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

    let mut final_tiles = Vec::new();
    for k in handles {
       let mut t =  k.join().unwrap();
        final_tiles.append(&mut t);
    }

    println!("1- program took: {}ms", start.elapsed().as_millis());
    for t in final_tiles{
        t.write_data(&mut data, w, h);
    }

    println!("2- merging tiles took: {}ms", start.elapsed().as_millis());

    let mut img : image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::from_raw(w as u32, h as u32, data).ok_or("Error Creating buffer").unwrap();
    img = image::imageops::resize(&img, ((w as f32) * 1.) as u32, ((h as f32) * 1.0) as u32, image::imageops::FilterType::Lanczos3);
    img = image::imageops::flip_vertical(&img);
    img.save("test.png").unwrap();
    
    println!("3- writing file to disk took: {}ms", start.elapsed().as_millis());
}
