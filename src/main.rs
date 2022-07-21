use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = app.time * 5. as f32;
    let draw = app.draw();
    let size = app.window_rect();
    for i in 0..20 {
        let start_point = pt2(x1(t + i as f32), y1(t + i as f32));
        let end_point = pt2(x2(t + i as f32), y2(t + i as f32));
        draw.line()
            .color(STEELBLUE)
            //.rgba(0.2, 0.7, 0.7, 0.9)
            .start(start_point)
            .end(end_point)
            .weight(3.0)
            .caps_round();

        draw.line()
            .color(PLUM)
            .start(-start_point)
            .end(-end_point)
            .weight(3.0)
            .caps_round();

        //  draw.background().rgba(0., 0., 0., 0.5);
    }
    draw.rect().rgba(0., 0., 0., 0.25).w_h(size.w(), size.h());
    draw.to_frame(app, &frame).unwrap();
}

fn x1(t: f32) -> f32 {
    f32::sin(-t / 10.) * 100. + f32::sin(t / 5.) * (20. + t)
}

fn y1(t: f32) -> f32 {
    f32::cos(-t / 10.) * 100. + f32::sin(t / 5.) * 50.
}

fn x2(t: f32) -> f32 {
    f32::sin(t / 10.) * 200. + f32::sin(t) * 2.
}

fn y2(t: f32) -> f32 {
    f32::cos(t / 20.) * 200. + f32::cos(t / 12.) * 20.
}
