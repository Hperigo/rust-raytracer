use crate::vec::Vec3;
use crate::ray::Ray;


pub struct Camera{
	origin : Vec3, 
	lower_left_corner : Vec3,
	horizontal : Vec3, 
	vertical : Vec3,
	
	u : Vec3,
	v : Vec3,
	w : Vec3,
	lens_radius : f32,
}

impl Camera {

	pub fn new(look_from : Vec3, look_at: Vec3, 
			   vup : Vec3, vfov : f32,
			   
			   aspect_ratio : f32, aperture : f32, focus_dist : f32 ) -> Self{

		let theta = vfov * (std::f32::consts::PI / 180.0);
		let h = (theta/2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = viewport_height * aspect_ratio;	
		

		let w = Vec3::normalize(look_from - look_at);
		let u = Vec3::normalize( Vec3::cross(&vup, &w) );
		let v = Vec3::cross(&w, &u);


		let origin = look_from;
		let horizontal = focus_dist * u * viewport_width;
		let vertical = focus_dist * v * viewport_height;
		let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

		let lens_radius = aperture / 2.0;

		let camera = Camera { 
			origin,
			horizontal,
			vertical,
			lower_left_corner,
			
			u, v, w,
			lens_radius
		};

		camera
	}

	pub fn get_ray(&self, s : f32, t : f32 ) -> Ray {
		//Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
		let rd = self.lens_radius * Vec3::random_unit_circle();
		let offset = self.u * rd.x + self.v * rd.y;

		Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset )
	}
}