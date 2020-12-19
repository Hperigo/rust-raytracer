use crate::vec::Vec3;
use crate::ray::Ray;


pub struct Camera{
	origin : Vec3, 
	lower_left_corner : Vec3,
	horizontal : Vec3, 
	vertical : Vec3,
}



impl Camera {

	pub fn new( aspect_ratio : f32 ) -> Self{

		let viewport_height = 2.0;
		let viewport_width = viewport_height * aspect_ratio;	
		let focal_length = 1.0;

		let mut camera = Camera { 
			origin : Vec3::zero(),
			horizontal : Vec3::new( viewport_width, 0.0, 0.0),
			vertical : Vec3::new(0.0, viewport_height, 0.0 ),
			lower_left_corner : Vec3::zero(),
		};

		camera.lower_left_corner = camera.origin - camera.horizontal / 2.0 - camera.vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
		camera
	}

	pub fn get_ray(&self, u : f32, v : f32 ) -> Ray {
		Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
	}
}