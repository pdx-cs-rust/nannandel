use nannou::prelude::*;
use nannou::image::{DynamicImage, ImageBuffer, Rgba};

struct Model {
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let window = app.main_window();
    let (w, h) = window.rect().w_h();

    let img = ImageBuffer::from_fn(w.floor() as u32, h.floor() as u32, |x, y| {
        Rgba([
            ((x * y) % 256) as u8,
            (x % 256) as u8,
            (y % 256) as u8,
            255,
        ])
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

    draw.scale(100.0 * app.time)
        .polygon()
        .points_textured(&model.texture, points);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .simple_window(view)
        .run();
}
