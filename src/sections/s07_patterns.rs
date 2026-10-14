//! § 07 Patterns

pub fn if_let() {
    struct Temp { freezing: bool, degrees: i32 }

    fn describe(t: Temp) {
        if let Temp { freezing: true, degrees } = t {
            println!("Cold: {}°", degrees);
        } else if let Temp { freezing: false, degrees } = t {
            println!("Mild: {}°", degrees);
        }
    }

    // ---- check ----
    describe(Temp { freezing: true, degrees: -3 });
    describe(Temp { freezing: false, degrees: 18 });
}

pub fn match_arms_are_patterns() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }

    // ---- slide ----
    fn describe(t: Temp) {
        match t {
            Temp { freezing: true, degrees } => println!("Cold: {}°", degrees),
            Temp { freezing: false, degrees } => println!("Mild: {}°", degrees),
        }
    }

    // ---- check ----
    describe(Temp { freezing: true, degrees: -3 });
    describe(Temp { freezing: false, degrees: 18 });
}

pub fn matches_must_be_exhaustive() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    let temps = [
        Temp { freezing: true, degrees: 0 },
        Temp { freezing: false, degrees: 100 },
        Temp { freezing: false, degrees: 37 },
    ];
    for t in temps {

    // ---- slide ----
    match t {
        Temp { degrees: 0, .. } => println!("Exactly zero"),
        Temp { degrees: 100, .. } => println!("Boiling"),
        Temp { degrees, .. } => println!("{}°", degrees),
        // without this last arm: compile error, non-exhaustive patterns
    }

    // ---- setup (end of loop) ----
    }
}

pub fn the_catch_all_pattern() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }
    let temps = [
        Temp { freezing: true, degrees: 0 },
        Temp { freezing: false, degrees: 100 },
        Temp { freezing: false, degrees: 37 },
    ];
    for t in temps {

    // ---- slide ----
    match t.degrees {
        0 => println!("Exactly zero"),
        100 => println!("Boiling"),
        _ => println!("{}°", t.degrees),
    }

    // ---- setup (end of loop) ----
    }
}
