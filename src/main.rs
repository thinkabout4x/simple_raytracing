use simple_raytracing::{Image, Sphere};
fn main() {
    let mut picture = Image::new(1024,768, [1,10,255]);

    let sph = Sphere::new(2, [-3.,1.,-10.], [255,0,0]);
    picture.render(&sph);
    picture.save_image("sample_image.png");
}

