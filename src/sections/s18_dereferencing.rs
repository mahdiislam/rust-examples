//! § 18 Dereferencing
//!
//! Slide 18.2 "Explicit * needs Copy" is in `does_not_compile/`.

pub fn auto_deref() {
    struct Point {
        x: f64,
        y: f64,
    }

    fn main() {
        let p = Point { x: 2.0, y: 5.0 };
        let r = &p;
        println!("({}, {})", r.x, r.y);
    }
    // prints (2, 5)

    // ---- check ----
    main();
    let p = Point { x: 2.0, y: 5.0 };
    assert_eq!(format!("({}, {})", (&p).x, (&p).y), "(2, 5)");
}

pub fn with_copy_it_works() {
    // ---- setup ----
    fn flip(p: Point) -> Point {
        Point { x: -p.x, y: -p.y }
    }

    // ---- slide ----
    #[derive(Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    fn main() {
        let p = Point { x: 2.0, y: 5.0 };
        let r = &p;
        flip(*r); // fine now: `*r` is copied
    }

    // ---- check ----
    main();
    let p = Point { x: 2.0, y: 5.0 };
    let r = &p;
    let flipped = flip(*r);
    println!("flip((2, 5)) = ({}, {})", flipped.x, flipped.y);
    assert_eq!((flipped.x, flipped.y), (-2.0, -5.0));
}
