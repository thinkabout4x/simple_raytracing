use std::{f32::consts::PI};
use image::{Rgb, RgbImage, Pixel};
use nalgebra::{Vector3, Vector2, vector};

pub struct Image{
    width: u32,
    height: u32,
    color: Rgb<u8>,
    image: RgbImage
}

pub struct Sphere{
    radius: u32,
    coordinates: Vector3<f32>, 
    color: Rgb<u8>,
    albedo: Vector2<f32>,
    specular_exponent: f32
}

pub struct Light {
    coordinates: Vector3<f32>,
    intensity: f32,
}

impl Image{
    pub fn new(width: u32, height: u32, color: [u8; 3]) -> Image{
        let mut img = Image{width, height, image: RgbImage::new(width, height), color: Rgb(color)};
        img.set_canvas_color(color);
        img
    }

    pub fn set_canvas_color(&mut self, color: [u8; 3]) {
        let color = Rgb(color);
        for (_, _, pixel) in self.image.enumerate_pixels_mut(){
            *pixel = color;
        }
    }

    pub fn render(&mut self, spheres: &Vec<Sphere>,  lights: &Vec<Light>){
        let fov = PI/2.;
        let origin = vector![0.,0.,0.];
        for (x, y, pixel) in self.image.enumerate_pixels_mut(){
            let x: f32 = (2.*((x as f32)+0.5)/(self.width as f32)-1.)*(fov/2.).tan()*(self.width as f32)/(self.height as f32);
            let y: f32 = -(2.*((y as f32)+0.5)/(self.height as f32)-1.)*(fov/2.).tan();
            let dir: Vector3<f32> = vector![x,y, -1.].normalize(); 

            *pixel = cast_ray(&self.color, &spheres, &origin, &dir, &lights);

        }
    }

    pub fn save_image(&self, path: &str){
        self.image.save(path).unwrap();
    }
}

impl Sphere{
    pub fn new(radius: u32, coordinates: [f32; 3], color: [u8; 3], albedo: [f32; 2], specular_exponent: f32) -> Sphere{
        let coordinates = vector![coordinates[0], coordinates[1], coordinates[2]];
        let albedo = vector![albedo[0], albedo[1]];
        let color = Rgb(color);
        Sphere{ radius, coordinates, color, albedo, specular_exponent}
    }

    fn ray_intersect(&self, &origin: &Vector3<f32>, &dir: &Vector3<f32>) -> Option<f32>{
        let l: Vector3<f32> = self.coordinates - origin;
        let tca = l.dot(&dir);
        let d2 = l.dot(&l) - tca*tca;
        if d2 > (self.radius.pow(2)) as f32{
             return None;
        } else {
            let thc = (self.radius.pow(2) as f32 - d2).sqrt();
            let mut t0 = tca - thc;
            let t1 = tca + thc;
            if t0 < 0.{
                t0 = t1;
                if t0 < 0.{
                    return None;
                } else {
                    return Some(t0);
                }
            } else {
                return Some(t0);
            }
        }

    }

}

impl Light{
    pub fn new(coordinates: [f32; 3], intensity: f32) -> Light{
        let coordinates = vector![coordinates[0], coordinates[1], coordinates[2]];
        Light{coordinates, intensity}
    }
}

fn cast_ray(canvas_color: &Rgb<u8>, spheres: &Vec<Sphere>, origin: &Vector3<f32>, dir: &Vector3<f32>, lights: &Vec<Light>) -> Rgb<u8>{
    let mut sphere_dist= f32::MAX;
    let mut pixel_color = *canvas_color;
    
    for sphere in spheres{
        match sphere.ray_intersect(&origin, &dir){
            Some(x) => {
                if x < sphere_dist{
                    sphere_dist = x;
                    let hit = origin + dir*sphere_dist;
                    let n = (hit - sphere.coordinates).normalize();
                    pixel_color = sphere.color;
                    pixel_color = diffuse_and_specular_light(&pixel_color, lights, &hit, &n, dir, sphere);
                    
                }
            }
            None => {}
        };
    }
    pixel_color
}

fn diffuse_and_specular_light(color: &Rgb<u8>, lights: &Vec<Light> ,point: &Vector3<f32>, normal: &Vector3<f32>, dir: &Vector3<f32>,sphere: &Sphere) -> Rgb<u8> {
    let mut diffuse_light_intensity: f32 = 0.;
    let mut specular_light_intensity: f32 = 0.;
    for light in lights{
        let light_dir = (light.coordinates-point).normalize();
        diffuse_light_intensity += light.intensity * f32::max(0.,light_dir.dot(normal));  
        specular_light_intensity += (f32::max(0.,reflect(&light_dir, normal).dot(dir))).powf(sphere.specular_exponent)*light.intensity;
    }
    let coeff = diffuse_light_intensity*sphere.albedo[0]+specular_light_intensity*sphere.albedo[1];

    let result = color.map(|mut x| {
        let value = (x as f32/255.)*(coeff);
        x = (value*(255. as f32)) as u8;
        x
    });
    result
}

fn reflect(i: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32>{
    return i-2.*n*(i.dot(n));
}