use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(400, 400).run();
}

fn view(app: &App) {
    let draw = app.draw();
    let win = app.window_rect();
    let center = pt2(0.0, 0.0);

    draw.background().color(BLACK);

    draw.ellipse().xy(center);
}
