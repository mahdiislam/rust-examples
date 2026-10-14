// Slide 18.2 · Explicit * needs Copy
//
// This program must NOT compile. rustc rejects it with:
//     error[E0507]: cannot move out of `*r` which is behind a shared reference
// `cargo test` checks that this is the only error code rustc reports.

struct Point {
    x: f64,
    y: f64,
}

fn flip(p: Point) -> Point {
    Point { x: -p.x, y: -p.y }
}

fn main() {
    let p = Point { x: 2.0, y: 5.0 };
    let r = &p;
    flip(*r);
    // error: cannot move out of `*r`
    //        which is behind a shared reference
}
