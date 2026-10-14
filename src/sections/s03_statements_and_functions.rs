//! § 03 Statements & functions

pub fn statements() {
    let a = 10;
    let b = 20;
    let sum = a + b;

    // ---- check ----
    println!("sum = {sum}");
    assert_eq!(sum, 30);
}

pub fn multi_line_statements() {
    let total: i32 = [3, 1, 4, 1, 5]
        .iter()
        .map(|n| n * 2)
        .sum();

    // ---- check ----
    println!("total = {total}");
    assert_eq!(total, 28);
}

pub fn a_function_with_no_return_value() {
    fn say_hello() {
        println!("Hello!");
    }

    // ---- check ----
    say_hello();
}

pub fn a_function_that_returns_a_value() {
    fn pick_number() -> u32 {
        7
    }

    // ---- check ----
    println!("pick_number() = {}", pick_number());
    assert_eq!(pick_number(), 7);
}
