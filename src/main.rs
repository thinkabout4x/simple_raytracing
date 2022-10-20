use simple_raytracing::{Image, Sphere};
fn main() {
    let mut picture = Image::new(256,256);

    let sph = Sphere::new(20, 50, 50, [0,0,0]);
    picture.set_canvas_color([1,10,255]);
    picture.draw_sphere(sph);
    picture.save_image("sample_image.png");
}

