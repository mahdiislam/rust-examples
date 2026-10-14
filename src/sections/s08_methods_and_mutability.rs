//! § 08 Methods & mutability
//!
//! Slides 08.3 and 08.4 ("Immutable by default") are in `does_not_compile/`.

pub fn declaring_methods() {
    struct Temp { freezing: bool, degrees: i32 }

    impl Temp {
        fn is_below_zero(self) -> bool {
            self.degrees < 0
        }
    }

    // ---- check ----
    let t = Temp { freezing: false, degrees: 4 };
    println!("4° below zero? {}", t.is_below_zero());
}

pub fn calling_methods() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    impl Temp {
        fn is_below_zero(self) -> bool {
            self.degrees < 0
        }
    }

    // ---- slide ----
    fn main() {
        let t = Temp { freezing: true, degrees: -5 };
        println!("below zero? {}", t.is_below_zero());
        // prints "below zero? true"
    }

    // ---- check ----
    main();
    assert!(Temp { freezing: true, degrees: -5 }.is_below_zero());
}

pub fn mut_() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }

    // ---- slide ----
    let mut t = Temp { freezing: false, degrees: 12 };
    t.degrees = 14; // fine

    // ---- check ----
    println!("t.degrees = {}", t.degrees);
    assert_eq!(t.degrees, 14);
}
