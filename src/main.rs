use simple_raytracing::{Image, Sphere, Light, Material};
fn main() {
    let mut picture = Image::new(1024,768);
    let envmap = Image::read("envmap.jpg");

    let ivory = Material::new([0.4,0.4,0.3], [0.6,0.3,0.1], 50.);
    let red_rubber = Material::new([0.3,0.1,0.1], [0.9,0.1,0.0], 10.);
    let mirror = Material::new([1.,1.,1.], [0.,10.,0.8], 1425.);

    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(2, [-3.,0.,-16.], &ivory));
    spheres.push(Sphere::new(2, [-1.,-1.5,-12.], &mirror));
    spheres.push(Sphere::new(3, [1.5,-0.5,-18.], &red_rubber));
    spheres.push(Sphere::new(4, [7.,5.,-18.], &mirror));


    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new([-20.,20.,20.], 1.5));
    lights.push(Light::new([30.,50.,-25.], 1.8));
    lights.push(Light::new([30.,20.,30.], 1.7));


    picture.render(&envmap, &spheres, &lights);

    picture.save_image("sample_image.png");
}

