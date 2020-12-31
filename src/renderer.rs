extern crate rand;
use rand::Rng;

use std::sync::*;

// use crate::color::Color;
use crate::color::Color;
use crate::vec::Vec3;
use crate::ray::Ray;

use crate::geometry::Sphere;

use crate::camera::Camera;

#[derive(Clone)]
pub struct HitRecord{
pub    p : Vec3,
pub   normal : Vec3,
pub    t : f32,
pub   front_face : bool,
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


pub struct HittableList {
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

/*
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
*/

fn ray_color(r : &Ray, hit_world : &HittableList, depth : i32) -> Vec3 {
    
    let mut rec = HitRecord::new();
    
    if depth <= 0 {
        return Vec3::zero();
    }

    if hit_world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return ray_color( &Ray::new(rec.p,  target - rec.p), hit_world, depth - 1) * 0.5;
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
