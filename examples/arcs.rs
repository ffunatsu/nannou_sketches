#![allow(unused)]

use nannou::{lyon, prelude::*};
use nannou::geom::path::{Path, Builder};
// use rand::rngs::StdRng;
// use rand::SeedableRng;
// use rand::RngExt;
use nannou::rand::rngs::StdRng;
use nannou::rand::SeedableRng;
use nannou::rand::RngExt;

fn main() {
    nannou::sketch(view).size(400, 400).run();
}

fn from_pt2(pt: Point2) -> lyon::math::Point {
    lyon::math::point(pt.x, pt.y)
}

fn arc(d: f32, ang_w: f32, center: Point2) -> Path {
    let mut builder = Builder::new().with_svg();
    builder.move_to(from_pt2(pt2(d, 0.0)));
    builder.arc(
        from_pt2(center),
        lyon::math::vector(d, d),
        lyon::math::Angle::degrees(ang_w),
        lyon::math::Angle::degrees(0.0),
    );
    let arc_path = builder.build();
    arc_path
}

fn view(app: &App) {
    let t = app.time();

    let draw = app.draw();
    let win = app.window_rect();
    let center = pt2(0.0, 0.0);

    let cd = 30f32;
    let fd = 50.0f32;
    let td = 30.0f32;
    let sw = 3f32;

    draw.background().color(BLACK);

    draw.ellipse().xy(center).wh(vec2(1.0, 1.0) * cd);

    let seed: u64 = 42;
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let n = 3;
    let m = 5;

    for k in 0..m {
        let r: f32 = rng.random_range(-1.0f32..1.0f32);

        for i in 0..n {
            let d = fd + td * k as f32;
            let ang_offset = rng.random_range(0.0f32..120f32);
            let ang = 360f32 / n as f32 * i as f32;
            let ang_w = 360f32 / n as f32 / 2.0f32;

            let arc_path = arc(d, ang_w, center);

            let rot = draw.rotate(deg_to_rad(ang + ang_offset + (t * r * 30.0f32)));

            rot.path()
                .stroke()
                .stroke_weight(sw)
                .color(WHITE)
                .events(arc_path.iter());
        }
    }
}
