extern crate rand;
use rand::Rng;

use std::sync::*;

// use crate::color::Color;
use crate::color::Color;
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::hitrecord::{HitRecord, Hittable, HittableList};


fn ray_color(r : &Ray, hit_world : &HittableList, depth : i32) -> Vec3 {
    
    let mut rec = HitRecord::new();
    
    if depth <= 0 {
        return Vec3::zero();
    }

    if hit_world.hit(r, 0.001, f32::INFINITY, &mut rec) {

        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero() );
        let mut attenuation = Vec3::one();

        if let Some(m)  = rec.material.clone() {
            if m.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, &hit_world,  depth - 1);
            }else{
                return Vec3::zero();
            }
        }
    }

    let unit_vector = Vec3::normalize(r.dir);
    let t = 0.5 * (unit_vector.y + 1.0);
    let v =  Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t; 
    v
}

pub struct RenderData {
    pub render_width : usize,
    pub render_height : usize,
    pub render_aspect_ratio : f32,
    
    pub max_depth : i32,
    pub hittable : HittableList,
    pub camera  : Camera,
}

impl RenderData{
    pub fn new() -> Self {
        RenderData{
            render_width : 0,
            render_height : 0,
            render_aspect_ratio : 0.,
            
            max_depth : 10,
            hittable : HittableList::new(),
            camera : Camera::new(1.0),
        }   
    }
}
type RenderDataHandle = RwLock<RenderData>;

#[derive(Debug)]
pub struct Tile{

    pub x : usize,
    pub y : usize,
    pub w : usize,
    pub h : usize,
    
    pub data : Vec<Color<u8>>,
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
        (self.x + x, self.y + y)
    }

    pub fn run<'a>(&mut self, render_data : &'a RenderDataHandle){
        self.data.resize(self.w * self.h, Color::black() );
        //println!("Running on thread-id: {:?}", std::thread::current().id() ); 

        let world = render_data.read().unwrap();
        
        let viewport_height = 2.0;
        let viewport_width = viewport_height * world.render_aspect_ratio;
        
        let _focal_length = 1.0;
        let _origin = Vec3::zero();
        let _horizontal = Vec3::new(viewport_width, 0., 0.);
        let _vertical = Vec3::new(0., viewport_height, 0.); 

        let mut rng = rand::thread_rng();

        for y in 0..self.h{
            for x in 0..self.w{
                let screen_pos = self.world_location_of_pixel(x, y);
                
                let mut pixel_sample = Vec3::zero();
                let num_of_samples = 16;
                for _ in 0..num_of_samples{

                    let u = (screen_pos.0 as f32 + rng.gen::<f32>() ) / (world.render_width as f32 - 1.0); 
                    let v = (screen_pos.1 as f32 + rng.gen::<f32>() ) / (world.render_height as f32  - 1.0); 
                    
                    let r = world.camera.get_ray(u, v);
                    pixel_sample = pixel_sample + ray_color(&r, &world.hittable, world.max_depth);
                }


                pixel_sample = pixel_sample * (1.0 / num_of_samples as f32);
                let index = x + y * self.w;
                self.data[index] = Color::new_from_f32(pixel_sample.x.sqrt(), pixel_sample.y.sqrt(), pixel_sample.z.sqrt() ); //Color::new_from_f32(red, green, 0.0);
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
