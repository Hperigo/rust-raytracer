use crate::vec::Vec3;
use crate::ray::Ray;
use crate::hitrecord::HitRecord;
use rand::Rng;

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

#[derive(Clone)]
pub struct Metal {
    pub albedo : Vec3,
    pub fuzz : f32,
}

impl Material for Metal {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Vec3, scattered : &mut Ray) -> bool{
        let reflected = Vec3::reflect( Vec3::normalize( r_in.dir),  rec.normal );
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere() );
        *attenuation = self.albedo;
        return true;
    }
}


#[derive(Clone)]
pub struct Dieletric {
    pub ir : f32,   
}

impl Dieletric {
    fn reflectance( cosine : f32, ref_idx : f32 )  -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dieletric {


    fn scatter(&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Vec3, scattered : &mut Ray) -> bool{
        
        let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };

        let unit_direction = Vec3::normalize(r_in.dir);

        let cos_theta = Vec3::dot( &(unit_direction * -1.0), &rec.normal ).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract  = refraction_ratio * sin_theta > 1.0;
        
        let mut rng = rand::thread_rng();
        
        let direction = 
            if cannot_refract || Dieletric::reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>() { 
                Vec3::reflect(unit_direction, rec.normal) } 
             else {
                Vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };
        
        *attenuation = Vec3::one();
        *scattered = Ray::new(rec.p, direction);
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