use image::{Rgb, RgbImage};

// pub fn image_creator(width: u32, height: u32, path: &str) {
//     let buffer = ImageBuffer::from_fn(width, height, |x, y| {
//         let u8_max = 255.0;
//         let r = (u8_max*(x as f64 / (width-1) as f64)) as u8;
//         let g = (u8_max*(y as f64 / (height-1) as f64)) as u8;
//         let b = (u8_max*0.0) as u8;

//         Rgb([r,g,b])
//     }); 

//     buffer.save(path).unwrap();
// }

pub struct Image{
    width: u32,
    height: u32,
    image: RgbImage
}

pub struct Point{
    x: u32,
    y: u32
}

pub struct Sphere{
    radius: u32,
    center: Point, 
    color: Rgb<u8>,
}

impl Image{
    pub fn new(width: u32, height: u32) -> Image{
        Image{width, height, image: RgbImage::new(width, height)}
    }

    pub fn set_canvas_color(&mut self, color: [u8; 3]) {
        let color = Rgb(color);
        for (_, _, pixel) in self.image.enumerate_pixels_mut(){
            *pixel = color;
        }
    }

    pub fn draw_sphere(&mut self, sphere: Sphere){
        for (x, y, pixel) in self.image.enumerate_pixels_mut(){
            if (x as i32-sphere.center.x as i32).pow(2)+(y as i32-sphere.center.y as i32).pow(2) <= (sphere.radius.pow(2)) as i32{
                *pixel = sphere.color;
            }
        }
    }

    pub fn save_image(&self, path: &str){
        self.image.save(path).unwrap();
    }
}

impl Sphere{
    pub fn new(radius: u32, x: u32, y: u32, color: [u8; 3]) -> Sphere{
        let center = Point { x, y };
        let color = Rgb(color);
        Sphere{ radius, center, color}
    }

}