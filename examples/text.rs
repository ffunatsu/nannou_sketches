use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(400, 400).run();
}

fn view(app: &App) {
    let draw = app.draw();
    let t = app.time();

    let ts = vec!["T", "I", "H", "O"];

    draw.background().color(BLACK);

    for k in 0..4 {
        for i in 0..13 {
            let rot = draw
                .rotate(deg_to_rad(i as f32 * (360.0 / 12.0) + t * 20.0f32))
                .translate(vec3(100.0 + 30.0 * k as f32, k as f32 * 50.0f32, 0.0))
                ;
            rot
                .text(ts[k])
                .font_size(50)
                .color(WHITE);
        }
    }
}
