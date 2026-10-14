//! § 09 Traits
//!
//! Slide 09.2 "Orphan rules" has bullet points only; its text is in the catalog.

pub fn declaring_a_trait() {
    trait Describe {
        fn summary(self) -> String;
    }

    // ---- check ----
    impl Describe for u8 {
        fn summary(self) -> String {
            format!("the byte {self}")
        }
    }
    println!("{}", 7u8.summary());
}

pub fn our_trait_our_type() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    trait Describe {
        fn summary(self) -> String;
    }

    // ---- slide ----
    impl Describe for Temp {
        fn summary(self) -> String {
            format!("{}°", self.degrees)
        }
    }

    fn main() {
        let t = Temp { freezing: false, degrees: 21 };
        println!("{}", t.summary()); // "21°"
    }

    // ---- check ----
    main();
    assert_eq!(Temp { freezing: false, degrees: 21 }.summary(), "21°");
}

pub fn our_trait_a_foreign_type() {
    // ---- setup ----
    trait Describe {
        fn summary(self) -> String;
    }

    // ---- slide ----
    impl Describe for bool {
        fn summary(self) -> String {
            if self { "yes".into() } else { "no".into() }
        }
    }

    fn main() {
        println!("{}", true.summary()); // "yes"
    }

    // ---- check ----
    main();
    assert_eq!(true.summary(), "yes");
    assert_eq!(false.summary(), "no");
}

pub fn a_foreign_trait_our_type() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }

    // ---- slide ----
    impl std::ops::Neg for Temp {
        type Output = Temp;

        fn neg(self) -> Temp {
            Temp { degrees: -self.degrees, freezing: !self.freezing }
        }
    }

    fn main() {
        let t = Temp { freezing: false, degrees: 8 };
        let u = -t;                // only possible thanks to `Neg`
        println!("{}", u.degrees); // -8
    }

    // ---- check ----
    main();
    assert_eq!((-Temp { freezing: false, degrees: 8 }).degrees, -8);
}

pub fn self_() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }

    // ---- slide ----
    impl std::ops::Neg for Temp {
        type Output = Self;

        fn neg(self) -> Self {
            Self { degrees: -self.degrees, freezing: !self.freezing }
        }
    }

    // ---- check ----
    let u = -Temp { freezing: false, degrees: 8 };
    println!("u.degrees = {}, u.freezing = {}", u.degrees, u.freezing);
    assert_eq!((u.degrees, u.freezing), (-8, true));
}
