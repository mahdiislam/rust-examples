//! § 05 Fields, methods & modules

pub fn field_access() {
    // ---- setup ----
    struct User {
        name: String,
    }
    fn fetch_user() -> User {
        User { name: String::from("ferris") }
    }

    // ---- slide ----
    let size = (640, 480);
    size.0; // 640

    let user = fetch_user();
    user.name; // a field of a struct

    // ---- check ----
    println!("size.0 = {}", size.0);
    assert_eq!(size.0, 640);
}

pub fn method_calls() {
    let word = "ferris";
    word.len(); // 6

    // ---- check ----
    println!("\"{word}\".len() = {}", word.len());
    assert_eq!(word.len(), 6);
}

pub fn paths_with_double_colon() {
    // `std` is a crate, `cmp` a module, `max` a function
    let biggest = std::cmp::max(4, 9); // 9

    // ---- check ----
    println!("biggest = {biggest}");
    assert_eq!(biggest, 9);
}

pub fn use_brings_names_into_scope() {
    use std::cmp::max;

    let biggest = max(2, 5); // 5

    // ---- check ----
    println!("biggest = {biggest}");
    assert_eq!(biggest, 5);
}

// Importing the same names twice in one scope is an error, so each style gets its own block.
pub fn importing_several_names() {
    {
        // one per line:
        use std::cmp::min;
        use std::cmp::max;

        // ---- check ----
        assert_eq!((min(2, 5), max(2, 5)), (2, 5));
    }
    {
        // grouped:
        use std::cmp::{min, max};

        // ---- check ----
        assert_eq!((min(2, 5), max(2, 5)), (2, 5));
    }
    {
        // also valid:
        use std::{cmp::min, cmp::max};

        // ---- check ----
        assert_eq!((min(2, 5), max(2, 5)), (2, 5));
        println!("min(2, 5) = {}, max(2, 5) = {}", min(2, 5), max(2, 5));
    }
}

pub fn glob_imports() {
    // brings `min`, `max`, `Ordering`, and more into scope
    use std::cmp::*;

    // ---- check ----
    println!("max(3, 8) = {}, 1.cmp(&2) = {:?}", max(3, 8), 1.cmp(&2));
    assert_eq!(1.cmp(&2), Ordering::Less);
}

pub fn types_are_namespaces_too() {
    let n = "crab".len();     // 4
    // ---- check ----
    assert_eq!(n, 4);

    let n = str::len("crab"); // also 4
    // ---- check ----
    assert_eq!(n, 4);
    println!("both ways give {n}");
}

pub fn the_prelude() {
    // `String` is an ordinary struct, not a primitive type
    let s = String::new();

    // exactly the same, with the full path
    let s = std::string::String::new();

    // ---- check ----
    println!("an empty String has length {}", s.len());
    assert!(s.is_empty());
}

pub fn what_the_prelude_does() {
    use std::prelude::rust_2021::*;
    // re-exports `Vec`, `String`, `Option`, `Result`, `Box`, ...

    // ---- check ----
    let v: Vec<Option<Box<str>>> = vec![Some("boxed".into()), None];
    println!("v = {v:?}");
}
