use simple_raytracing::{Image, Sphere, Light};
fn main() {
    let mut picture = Image::new(1024,768, [50,180,200]);
    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(2, [-3.,0.,-16.], [100,100,75]));
    spheres.push(Sphere::new(2, [-1.,-1.5,-12.], [75,25,25]));
    spheres.push(Sphere::new(3, [1.5,-0.5,-18.], [75,25,25]));
    spheres.push(Sphere::new(4, [7.,5.,-18.], [100,100,75]));


    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new([-20.,20.,20.], 1.5));

    picture.render(&spheres, &lights);

    picture.save_image("sample_image.png");
}

