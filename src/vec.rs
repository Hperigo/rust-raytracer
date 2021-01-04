extern crate rand;
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3{
    pub x: f32, 
    pub y : f32,
    pub z : f32,
}

impl Vec3{
     pub fn zero()->Self{
         Vec3{
             x: 0., y: 0., z: 0.
         }
     }
     
     #[allow(dead_code)]
     pub fn one()->Self{
        Vec3{x: 1., y: 1., z: 1.} 
     }

     pub fn new(x : f32, y : f32, z : f32 )->Self{
        Vec3{x,y,z}
     }

     pub fn dot( u : &Vec3, v : &Vec3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
     }

    pub fn normalize( u : Vec3) -> Vec3 {
        u / u.length() 
    }   
    
    #[allow(dead_code)]
     pub fn cross( u : &Vec3, v : &Vec3) -> Vec3 {
        Vec3{
            x : u.y * v.z - u.z * v.y, 
            y : u.z * v.x - u.x * v.z,
            z : u.x * v.y - u.y * v.x,
        }
     }

     pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
     }
    
     pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
     }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new( rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>() )
    }
    
    pub fn random_in_range(min : f32, max : f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new( rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max) )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
       loop {
         let p = Vec3::random();
         if p.length_squared() < 1.0  {
             return p;
         }
       }    
    }

    pub fn reflect( v : Vec3, n : Vec3 ) -> Vec3{
        v - 2.0 * Vec3::dot(&v,  &n) * n
    }

    pub fn refract( uv : Vec3, n : Vec3, etai_over_etat : f32 ) -> Vec3{
        let cos_theta = Vec3::dot( &(uv * -1.0), &n ).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = ((1.0 - r_out_perp.length_squared()).sqrt() * -1.0) * n;
        r_out_perp + r_out_parallel
    }

    pub fn random_unit_vector() -> Vec3{
        Vec3::normalize(Vec3::random_in_unit_sphere())
    }

    pub fn random_unit_circle() -> Vec3 {
        let mut rng = rand::thread_rng();
        
        loop {
            let p = Vec3::new( rng.gen::<f32>(), rng.gen::<f32>(), 0.0 );
            if p.length_squared() > 1.0 { continue; };

            return p; 
        };
    }

    pub fn near_zero(&self) -> bool {
        let m = std::f32::MIN;
        self.x.abs() < m && self.y.abs() < m && self.z.abs() < m
    }
}


// TRAIT implementations -----
impl std::ops::Add<f32> for Vec3{
    type Output = Vec3;

    fn add(self, rhs : f32) -> Vec3{ 
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, rhs : Vec3) -> Vec3{ 
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vec3> for f32{
    type Output = Vec3;

    fn sub(self, rhs : Vec3) -> Vec3{ 
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl std::ops::Sub<f32> for Vec3{
    type Output = Vec3;

    fn sub(self, rhs : f32) -> Vec3{ 
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3{
    type Output = Vec3;

    fn sub(self, rhs : Vec3) -> Vec3{ 
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f32{
    type Output = Vec3;

    fn mul(self, rhs : Vec3) -> Vec3{ 
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3{
    type Output = Vec3;

    fn mul(self, rhs : f32) -> Vec3{ 
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3{
    type Output = Vec3;

    fn mul(self, rhs : Vec3) -> Vec3{ 
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Div<f32> for Vec3{
    type Output = Vec3;

    fn div(self, rhs : f32) -> Vec3{ 
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Div<Vec3> for Vec3{
    type Output = Vec3;

    fn div(self, rhs : Vec3) -> Vec3{ 
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn vector_operators(){
        let a = Vec3::one();
        let b = Vec3::one();
        
        let c = a + b + 1.0;
        assert_eq!(c.x, 3.0);
        let d = c - 3.0;
        assert_eq!(d.x, 0.0);
        let e = a * 10.0;
        assert_eq!(e.x, 10.0);
        let f = e / 5.0;
        assert_eq!(f.x, 2.0);
    }

    #[test]
    fn vector_methods(){
       
        let length_squared_vec = Vec3::new(10., 0., 0.);
        assert_eq!(length_squared_vec.length_squared(), 100.);

        assert_eq!(length_squared_vec.length(), 10.);
        
        let a = Vec3::new(3.  , -3., 1.);
        let c = Vec3::new(-12., 12., -4.); 

        assert_eq!( Vec3::cross(&a, &c), Vec3::zero() );
    }
}
