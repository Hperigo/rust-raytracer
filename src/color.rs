#[derive(Debug, Clone)]
pub struct Color<T>{
    pub r : T,
    pub g : T,
    pub b : T,
}

impl Color<u8>{

    pub fn new_from_f32( r: f32, g : f32, b : f32 ) -> Self{
        Color{ 
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }   
    }

    pub fn black() -> Self{
        Color{
            r:  0,
            g : 0,
            b:  0
        }
    }
    
    #[allow(dead_code)]
    pub fn white() -> Self {
         Color{
            r:  255,
            g : 255,
            b:  255
        } 
    }

    pub fn red() -> Self{
        Color{ r:  255, g : 0, b:  0, }        
    }

    #[allow(dead_code)]
    pub fn green() -> Self{
        Color{ r: 0, g : 255, b:  0, }
    }
    
    #[allow(dead_code)]
    pub fn blue() -> Self{
        Color{ r:  0, g : 0, b:  255, }
    }
}


