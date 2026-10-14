//! § 06 Structs

pub fn declaring_a_struct() {
    struct Size {
        width: u32,
        height: u32,
    }

    // ---- check ----
    let s = Size { width: 4, height: 3 };
    println!("{} x {}", s.width, s.height);
}

pub fn struct_literals() {
    // ---- setup ----
    struct Size {
        width: u32,
        height: u32,
    }

    // ---- slide ----
    let a = Size { width: 800, height: 600 };
    let b = Size { height: 1080, width: 1920 };

    // ---- check ----
    println!("a = {} x {}, b = {} x {}", a.width, a.height, b.width, b.height);
    assert_eq!((b.width, b.height), (1920, 1080));
}

pub fn struct_update_syntax() {
    // ---- setup ----
    struct Size {
        width: u32,
        height: u32,
    }
    let b = Size { height: 1080, width: 1920 };

    // ---- slide ----
    let c = Size {
        width: 1024,
        ..b
    };

    // ---- check ----
    println!("c = {} x {}", c.width, c.height);
    assert_eq!((c.width, c.height), (1024, 1080));
}

pub fn update_syntax_for_every_field() {
    // ---- setup ----
    struct Size {
        width: u32,
        height: u32,
    }
    let c = Size { width: 1024, height: 1080 };

    // ---- slide ----
    let d = Size { ..c };

    // ---- check ----
    println!("d = {} x {}", d.width, d.height);
    assert_eq!((d.width, d.height), (1024, 1080));
}

pub fn destructuring_a_struct() {
    // ---- setup ----
    struct Size {
        width: u32,
        height: u32,
    }

    // ---- slide ----
    let s = Size { width: 300, height: 200 };
    let Size { width, height } = s;
    // `width` is 300, `height` is 200

    // ---- check ----
    println!("width = {width}, height = {height}");
    assert_eq!((width, height), (300, 200));
}

pub fn ignoring_the_rest() {
    // ---- setup ----
    struct Size {
        width: u32,
        height: u32,
    }
    let s = Size { width: 300, height: 200 };

    // ---- slide ----
    let Size { width, .. } = s;
    // `s.height` is ignored

    // ---- check ----
    println!("width = {width}");
    assert_eq!(width, 300);
}
