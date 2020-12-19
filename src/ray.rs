
use crate::vec::Vec3;

pub struct Ray {
   pub origin : Vec3,
   pub dir : Vec3,
}


impl Ray{

    pub fn new(origin : Vec3, dir : Vec3) -> Self {
        Ray {
            origin,
            dir,
        }
    }

    #[allow(dead_code)]
    pub fn at(&self, t : f32) -> Vec3{
        return self.origin + self.dir * t;
    }
}



#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn ray(){
        let r = Ray{
            origin : Vec3::zero(),
            dir : Vec3::new(0., 0., 1.0),
        };


        let p = r.at(0.5);
        assert_eq!(p, Vec3::new(0., 0., 0.5)); 
    }

}
