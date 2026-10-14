//! § 01 Variable bindings
//!
//! Slide 01.5 "Uninitialized: not allowed" is in `does_not_compile/`.

pub fn let_declare_then_assign() {
    let score;     // declare `score`
    score = 100;   // assign 100 to it

    // ---- check ----
    println!("score = {score}");
    assert_eq!(score, 100);
}

pub fn let_on_one_line() {
    let score = 100;

    // ---- check ----
    println!("score = {score}");
    assert_eq!(score, 100);
}

pub fn type_annotations() {
    let score: u8;  // `u8` is an unsigned 8-bit integer
    score = 100;

    // signed:   i8, i16, i32, i64, i128, isize
    // unsigned: u8, u16, u32, u64, u128, usize

    // ---- check ----
    println!("score = {score}, and a u8 holds 0 to {}", u8::MAX);
}

pub fn annotated_on_one_line() {
    let ratio: f32 = 0.5;

    // ---- check ----
    println!("ratio = {ratio}");
    assert_eq!(ratio * 2.0, 1.0);
}

pub fn uninitialized_assign_first() {
    // ---- setup ----
    fn report(n: i64) {
        println!("report({n})");
    }

    // ---- slide ----
    let total;
    total = 3;
    report(total); // `total`'s type is inferred from `report`'s signature

    // ---- check ----
    let _: i64 = total; // compiles only because `total` was inferred as i64
}

pub fn throwing_values_away() {
    // ---- setup ----
    fn load_config() -> String {
        println!("load_config() ran");
        String::from("verbose = true")
    }

    // ---- slide ----
    // a constant thrown away: does nothing at all
    let _ = 7;

    // the call still runs, its return value is dropped
    let _ = load_config();
}

pub fn underscore_prefixed_names() {
    // ---- setup ----
    fn compute() -> u32 {
        42
    }

    // ---- slide ----
    // not used yet, but we don't want a warning while we work on it
    let _draft = compute();

    // ---- check ----
    println!("_draft = {_draft} (it is a real binding)");
    assert_eq!(_draft, 42);
}

pub fn shadowing() {
    let level = 1;
    let level = level * 10;
    // from here on, `level` refers to the second binding (10).
    // the first one still lives until the end of the scope,
    // it just can't be reached by name anymore.

    // ---- check ----
    println!("level = {level}");
    assert_eq!(level, 10);
}
