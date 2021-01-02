use crate::vec::Vec3;
use crate::ray::Ray;
use crate::materials::{Material, Lambertian, Metal, Dieletric};
use crate::geometry::Sphere;
#[derive(Clone)]
pub struct HitRecord{
    pub    p         : Vec3,
    pub   normal     : Vec3,
    pub   t          : f32,
    pub   front_face : bool,
    pub   material   : Option<std::boxed::Box<dyn Material>>,
}


impl HitRecord{
    pub fn new()->Self{
        HitRecord{
            p : Vec3::zero(),
            normal : Vec3::zero(),
            t : 0.0,
            front_face : true,
            material : None, //std::boxed::Box::new( Lambertian{ albedo : Vec3::new(0.0, 0.0, 0.2) } ),
        }
    }
    pub fn set_face_normal(&mut self, r : &Ray, outward_normal : &Vec3){
        self.front_face = Vec3::dot(&r.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face  { *outward_normal } else { *outward_normal * -1.0 }
    }
} 

pub trait Hittable{
    fn hit(&self, ray : &Ray, t_min : f32, t_max : f32, hit_record : &mut HitRecord ) -> bool;
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

        // floor 
        table.objects.push( Box::new( Sphere::new( Vec3::new(0.0, -100.5, -1.0), 100.0,  Box::new( Lambertian{ albedo :  Vec3::new(0.7, 0.8, 0.4) } ) )));

        //spheres ---
        table.objects.push( Box::new( Sphere::new( Vec3::new(-1.0, 0.0, -1.0),0.5, Box::new( Dieletric{ ir : 0.9 } ) )));  
        table.objects.push( Box::new( Sphere::new( Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new( Lambertian{ albedo :  Vec3::new(0.4, 0.4, 0.4) } ) )));  
        table.objects.push( Box::new( Sphere::new( Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new( Metal{ albedo :  Vec3::new(0.9, 0.8, 0.4), fuzz : 0.9 } ) )));  

        return table;
    }
}