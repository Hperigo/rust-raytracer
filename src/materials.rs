use crate::vec::Vec3;
use crate::ray::Ray;
use crate::renderer::HitRecord;

pub trait Material {
   fn scatter(&self, r_in : &Ray, rec : HitRecord, attenuation : &mut Vec3, scattered : &mut Ray) -> bool;

}


pub struct Lambertian { 
    pub albedo : Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in : &Ray, rec : HitRecord, attenuation : &mut Vec3, scattered : &mut Ray) -> bool{
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction); 
        *attenuation = self.albedo; 
        return true;
    }
}

