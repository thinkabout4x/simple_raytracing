use std::{f32::consts::PI};
use image::{Rgb, RgbImage};
use nalgebra::{Vector3, vector};

pub struct Image{
    width: u32,
    height: u32,
    color: Vector3<f32>,
    image: RgbImage
}

pub struct Sphere<'a>{
    radius: u32,
    coordinates: Vector3<f32>, 
    material: &'a Material
}

pub struct Light {
    coordinates: Vector3<f32>,
    intensity: f32,
}

pub struct Material{
    color: Vector3<f32>,
    albedo: Vector3<f32>,
    specular_exponent: f32
}

impl Image{
    pub fn new(width: u32, height: u32, color: [f32; 3]) -> Image{
        let color = vector![color[0], color[1], color[2]];
        let color_rgb = to_rgb(color);
        let mut img = Image{width, height, image: RgbImage::new(width, height), color};
        img.set_canvas_color(color_rgb);
        img
    }

    fn set_canvas_color(&mut self, color: Rgb<u8>) {
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

            *pixel = to_rgb(cast_ray(&self.color, &spheres, &origin, &dir, &lights));

        }
    }

    pub fn save_image(&self, path: &str){
        self.image.save(path).unwrap();
    }
}

impl<'a> Sphere<'a>{
    pub fn new(radius: u32, coordinates: [f32; 3], material: &Material) -> Sphere{
        let coordinates = vector![coordinates[0], coordinates[1], coordinates[2]];
        Sphere{ radius, coordinates, material}
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

impl Material{
    pub fn new(color: [f32; 3], albedo: [f32; 3], specular_exponent: f32) -> Material{
        let color = vector![color[0], color[1], color[2]];
        let albedo = vector![albedo[0], albedo[1], albedo[2]];
        Material{color, albedo, specular_exponent}
    }
}

fn cast_ray(canvas_color: &Vector3<f32>, spheres: &Vec<Sphere>, origin: &Vector3<f32>, dir: &Vector3<f32>, lights: &Vec<Light>) -> Vector3<f32>{
    let mut pixel_color = *canvas_color;

    match ray_spheres_intersection(&origin, &dir, &spheres) {
        Some((hit,n, material)) => {
            let point = hit;
            let normal = n;

            let reflect_dir = reflect(dir, &normal);
            let reflect_orig = if reflect_dir.dot(&normal) < 0. {
                point - normal*0.001
            } else {
                point + normal*0.001
            };
            let reflect_color = cast_ray(canvas_color, spheres, &reflect_orig, &reflect_dir, lights);

            let mut diffuse_light_intensity: f32 = 0.;
            let mut specular_light_intensity: f32 = 0.;
            let color = material.color;

            for light in lights{
                let light_dir = (light.coordinates-point).normalize();
                let light_distance = (light.coordinates-point).norm();
                let shadow_orig = if light_dir.dot(&normal) < 0. {
                    point - normal*0.001
                } else {
                    point + normal*0.001
                };

                match ray_spheres_intersection(&shadow_orig, &light_dir, spheres){
                    Some((shadow_pt, _,_)) => {
                        if (shadow_pt- shadow_orig).norm() < light_distance{
                            continue;
                        }
                    }
                    None => {}
                }
                diffuse_light_intensity += light.intensity * f32::max(0.,light_dir.dot(&normal));  
                specular_light_intensity += (f32::max(0.,reflect(&light_dir, &normal).dot(dir))).powf(material.specular_exponent)*light.intensity;
            }
            let unit_vector: Vector3<f32> = vector![1.,1.,1.];
            pixel_color = color*diffuse_light_intensity*material.albedo[0]+unit_vector*specular_light_intensity*material.albedo[1]+reflect_color*material.albedo[2];
        }
        None => {}
    };
    pixel_color
}

fn ray_spheres_intersection<'a>(origin : &Vector3<f32>, dir:  &Vector3<f32>, spheres: &'a Vec<Sphere>) -> Option<(Vector3<f32>, Vector3<f32>, &'a Material)> {
    let mut sphere_dist= f32::MAX;
    let mut result = Option::None;
    for sphere in spheres{
        match sphere.ray_intersect(&origin, &dir){
            Some(x) => {
                if x < sphere_dist{
                    sphere_dist = x;
                    let hit = origin + dir*sphere_dist;
                    let n = (hit - sphere.coordinates).normalize();
                    result = Some((hit,n, sphere.material));
                }
            }
            None => {}
        };
    }
    return result;
}

fn reflect(i: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32>{
    return i-2.*n*(i.dot(n));
}

fn to_rgb(norm_color : Vector3<f32>) -> Rgb<u8>{
    let pixel_color = norm_color.map(|x| {
    (x*(255. as f32)) as u8
    });
    Rgb([pixel_color[0], pixel_color[1], pixel_color[2]])
}