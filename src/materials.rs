use crate::vec::Vec3;
use crate::ray::Ray;
use crate::renderer::HitRecord;


pub trait Material : MaterialClone {
   fn scatter(&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Vec3, scattered : &mut Ray) -> bool;
}

#[derive(Clone)]
pub struct Lambertian { 
    pub albedo : Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in : &Ray, rec : &HitRecord, attenuation : &mut Vec3, scattered : &mut Ray) -> bool{
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();    
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        *scattered = Ray::new(rec.p, scatter_direction); 
        *attenuation = self.albedo; 
        return true;
    }
}







// Trait impl
pub trait  MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}