use nannou::prelude::*;

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let win = app.window_rect();
    let x_y = win.wh() / 8.0;
    let w_h = win.wh() / 2.0;
    let draw = app.draw();
    draw.background().color(PURPLE);
    draw.rect()
        .x_y(x_y.x, x_y.y)
        .w_h(w_h.x, w_h.y)
        .z_degrees(app.time * 50.0)
        .color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
