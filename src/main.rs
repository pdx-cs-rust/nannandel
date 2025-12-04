use nannou::prelude::*;
use nannou::image::{DynamicImage, ImageBuffer, Rgba};
use num::Complex;

struct Model {
    texture: wgpu::Texture,
}

// Thanks to Jim Blandy for the initial implementation of this function.
fn mandel(x: f32, y: f32) -> u8 {
    let c = Complex::new(x, y);
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..255 {
        if z.norm_sqr() > 4.0 {
            return i
        }
        z = z * z + c;
    }

    255
}


fn model(app: &App) -> Model {
    let window = app.main_window();
    let (w, h) = window.rect().w_h();

    let img = ImageBuffer::from_fn(w.floor() as u32, h.floor() as u32, |x, y| {
        let m = mandel(x as f32 / w - 0.2, y as f32 / h);
        Rgba([m, m, m, 255])
    });

    let dynamic_img = DynamicImage::ImageRgba8(img);
    let texture = wgpu::Texture::from_image(app, &dynamic_img);
    Model { texture }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let rect = geom::Rect::from_wh(vec2(1.0, 1.0));
    let ellipse = geom::Ellipse::new(rect, 100.0).circumference();
    let points = ellipse.map(|p| {
        let point = Point2::from(p);
        let tex_coords = [point.x + 0.5, 1.0 - (point.y + 0.5)];
        (point, tex_coords)
    });

    let t = app.time;
    draw.scale(100.0 * t)
        .polygon()
        .points_textured(&model.texture, points)
        .rotate(t);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .simple_window(view)
        .run();
}
