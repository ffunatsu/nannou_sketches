use nannou::prelude::*;

// check for https://github.com/nannou-org/nannou/issues/1095

fn main() {
    nannou::sketch(view).size(800, 400).run();
}

fn view(app: &App) {
    let draw = app.draw();

    draw.background().color(BLACK);

    draw.ellipse()
        .no_fill()
        .stroke_weight(70.0)
        .stroke(WHITE)
        .x_y(-170.0, 0.0)
        .w_h(200.0, 200.0)
        .resolution(128.0);

    draw.ellipse()
        .no_fill()
        .stroke_weight(70.0)
        .stroke(WHITE)
        .x_y(170.0, 0.0)
        .w_h(200.0, 200.0);
}
