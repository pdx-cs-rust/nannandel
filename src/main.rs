use nannou::prelude::*;

struct Model {
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("nature_1.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    Model { texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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
        //        .update(update)
        .simple_window(view)
        .run();
}
