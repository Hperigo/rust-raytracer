use crate::vec::Vec3;
use crate::materials::{Material};
use crate::hitrecord::{HitRecord, Hittable};
use crate::ray::Ray;
pub struct Sphere {
    pub center : Vec3,
    pub radius : f32,
    pub material : Box<dyn Material + Send + Sync>,
}

impl Sphere{ 
    pub fn new(center : Vec3, radius : f32,  material : Box<dyn Material + Send + Sync>) -> Self{
        Sphere{
            center,
            radius,
            material : material, 
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
        hit_record.material =  Some(self.material.clone_box());
        return true;
    }
}