//! § 10 Copy, move & borrow
//!
//! Slides 10.3 "Non-Copy types are moved" and 10.4 "Moving into a function"
//! are in `does_not_compile/`.

pub fn copy_types_are_copied() {
    let a: u64 = 9;
    let b = a; // `a` is copied
    let c = a; // copied again, `a` is still usable

    // ---- check ----
    println!("a = {a}, b = {b}, c = {c}");
    assert_eq!(a + b + c, 27);
}

pub fn copy_into_functions() {
    fn show(x: u64) {
        println!("x = {}", x);
    }

    fn main() {
        let a: u64 = 9;
        show(a); // copied
        show(a); // copied again
    }

    // ---- check ----
    main();
}

pub fn borrowing_with_ref() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }

    // ---- slide ----
    fn show_temp(t: &Temp) {
        println!("{}°", t.degrees);
    }

    fn main() {
        let t = Temp { freezing: false, degrees: 3 };
        show_temp(&t); // borrowed for the duration of the call
        show_temp(&t); // borrowed again
    }

    // ---- check ----
    main();
}

pub fn borrowing_with_mut() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    fn show_temp(t: &Temp) {
        println!("{}°", t.degrees);
    }

    // ---- slide ----
    fn warm_up(t: &mut Temp) {
        t.degrees += 10;
    }

    fn main() {
        let mut t = Temp { freezing: true, degrees: -4 };
        show_temp(&t);
        warm_up(&mut t); // borrowed mutably
        show_temp(&t);
    }

    // ---- check ----
    main();
    let mut t = Temp { freezing: true, degrees: -4 };
    warm_up(&mut t);
    assert_eq!(t.degrees, 6);
}

pub fn receivers_by_reference() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }

    // ---- slide ----
    impl Clone for Temp {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }

    // ---- check ----
    let t = Temp { freezing: false, degrees: 3 };
    let u = t.clone();
    println!("cloned: {}°", u.degrees);
    assert_eq!((u.degrees, u.freezing), (3, false));
}

pub fn implicit_borrowing_of_the_receiver() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    impl Clone for Temp {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }
    fn show_temp(t: &Temp) {
        println!("{}°", t.degrees);
    }

    // ---- slide ----
    fn main() {
        let t = Temp { freezing: false, degrees: 3 };
        let mut u = t.clone();
        u.degrees += 20;

        show_temp(&t);
        show_temp(&u);
    }

    // ---- check ----
    main();
}

pub fn method_call_function_call() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    impl Clone for Temp {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }
    let t = Temp { freezing: false, degrees: 3 };

    // ---- slide ----
    let u = t.clone();

    let u = Clone::clone(&t);

    // ---- check ----
    println!("u.degrees = {}", u.degrees);
    assert_eq!(u.degrees, t.degrees);
}

pub fn implementing_copy() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }

    // ---- slide ----
    // `Copy` requires `Clone`
    impl Clone for Temp {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }

    impl Copy for Temp {}

    // ---- check ----
    let t = Temp { freezing: false, degrees: 3 };
    let u = t;
    println!("t is still usable after `let u = t;`: {}°", t.degrees);
}

pub fn clone_still_works() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    impl Clone for Temp {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }
    impl Copy for Temp {}

    // ---- slide ----
    let t = Temp { freezing: false, degrees: 3 };
    let u = t.clone();
    let v = t.clone();

    // ---- check ----
    println!("u = {}°, v = {}°", u.degrees, v.degrees);
}

pub fn and_nothing_is_moved_anymore() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    impl Clone for Temp {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }
    impl Copy for Temp {}

    // ---- slide ----
    let t = Temp { freezing: false, degrees: 3 };
    let u = t; // a copy of `t`
    let v = t; // another copy: `t` was never moved or borrowed

    // ---- check ----
    println!("t = {}°, u = {}°, v = {}°", t.degrees, u.degrees, v.degrees);
}

pub fn deriving_traits() {
    #[derive(Clone, Copy)]
    struct Temp {
        freezing: bool,
        degrees: i32,
    }
    // expands to `impl Clone for Temp` and `impl Copy for Temp`

    // ---- check ----
    let t = Temp { freezing: true, degrees: -1 };
    let u = t;
    let v = t;
    println!("three copies: {}°, {}°, {}°", t.degrees, u.degrees, v.degrees);
}
