
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


}

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
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
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
