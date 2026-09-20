#![allow(unused)]

use nannou::{lyon, prelude::*};
use nannou::geom::path::{Path, Builder};

fn main() {
    nannou::sketch(view).size(400, 400).run();
}

fn from_pt2(pt: Point2) -> lyon::math::Point {
    lyon::math::point(pt.x, pt.y)
}

fn polar_to_rect(r: f32, angle_rad: f32) -> Point2 {
    pt2(r * angle_rad.cos(), r * angle_rad.sin())
}

fn hexa(d: f32, center: Point2) -> Path {
    let mut builder = Builder::new().with_svg();
    // builder.move_to(from_pt2(pt2(d, 0.0)));
    for i in 0..7 {
        let p = polar_to_rect(d, (i as f32 / 6.0f32) * PI * 2.0f32);
        if i == 0 {
            builder.move_to(from_pt2(p + center));
        } else {
            builder.line_to(from_pt2(p + center));
        }
        // println!("{:}", p);
    }
    builder.build()
}

fn view(app: &App) {
    let t = app.time();

    let draw = app.draw();
    let win = app.window_rect();
    let center = pt2(0.0, 0.0);

    let cd = 30f32;
    let fd = 50.0f32;
    let td = 30.0f32;
    let sw = 2f32;

    draw.background().color(BLACK);

    // draw.rect().w_h(1.0, 1.0).color(WHITE);

    let rot = draw.rotate(t * 0.1f32);

    let n = 9;
    let m = 9;

    for k in 0..m {
        for i in 0..n {
            let d = 30.0f32;
            let d2 = 70.0f32;

            let x = (i as f32 - n as f32 / 2.0f32 + 0.5f32) * d2;
            let mut y = (k as f32 - m as f32 / 2.0f32 + 0.5f32) * d2;
            if i % 2 == 0 {
                y += (d2 - d);
            }
            let hexa_path = hexa(d, center + vec2(x, y));

            // let trans = draw.translate(vec3(x, y, 0.0));

            rot.path()
                .stroke()
                .stroke_weight(sw)
                .color(WHITE)
                .events(hexa_path.iter());
        }
    }
}
