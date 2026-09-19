use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(400, 400).run();
}

fn view(app: &App) {
    let draw = app.draw();

    draw.background().color(BLACK);
}
