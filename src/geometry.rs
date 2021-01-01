use crate::vec::Vec3;
use crate::materials::{Material, Lambertian};

pub struct Sphere {
    pub center : Vec3,
    pub radius : f32,
    pub material : Box<dyn Material + Send + Sync>,
}

impl Sphere{ 
    pub fn new(center : Vec3, radius : f32, albedo : Vec3) -> Self{
        Sphere{
            center,
            radius,
            material : Box::new( Lambertian { albedo } ), 
        }
    }
}

