//! Demonstrates Rust slices, ranges, and borrowing/lifetime behavior with practical examples.
//!
//! # Overview
//! - Shows how to take a slice from a `Vec<T>` using range syntax (`&v[start..end]`).
//! - Demonstrates common range forms (`a..`, `..b`, `..=b`, `a..b`) and membership checks.
//! - Explains that slices follow the same borrowing rules as all references.
//! - Illustrates explicit lifetime notation for slice-returning functions.
//! - Highlights why returning a slice from temporary data can be valid when backing data is static.
//! - Uses string slices to extract a file extension without allocating a new `String`.
//!
//! # Key ideas
//! - A slice is a borrowed view into contiguous data, not owned data.
//! - End bounds in `a..b` are exclusive; `a..=b` is inclusive.
//! - Returned slices must not outlive the source they borrow from.
//! - String slicing helpers like `rsplit_once` can return `&str` views efficiently.
//!
//! # Notes
//! - These examples include small runtime checks/assertions to verify expected behavior.
//! - Some borrow-checker error demonstrations are intentionally kept in a separate
//!   non-compiling section (`does_not_compile/`).
//! § 16 Slices
//!
//! Slides 16.6 and 16.8 are borrow-checker errors and live in `does_not_compile/`.

pub fn slices() {
    fn main() {
        let v = vec![10, 20, 30, 40, 50];
        let mid = &v[1..4];
        println!("mid = {:?}", mid);
    }
    // mid = [20, 30, 40]

    // ---- check ----
    main();
    let v = vec![10, 20, 30, 40, 50];
    assert_eq!(&v[1..4], [20, 30, 40]);
}

pub fn ranges() {
    fn main() {

        println!("{}", (5..).contains(&1000)); // true:  5 or more
        println!("{}", (..10).contains(&10));  // false: strictly below 10
        println!("{}", (..=10).contains(&10)); // true:  10 or below
        println!("{}", (1..4).contains(&2));   // true:  1, 2 or 3
    }

    // ---- check ----
    main();
    assert!((5..).contains(&1000));
    assert!(!(..10).contains(&10));
    assert!((..=10).contains(&10));
    assert!((1..4).contains(&2));
}

pub fn borrowing_rules_apply_to_slices() {
    fn rest(s: &[i32]) -> &[i32] {
        &s[1..]
    }

    fn main() {
        let arr = &[7, 8, 9];
        let tail = rest(arr);
        println!("tail = {:?}", tail); // [8, 9]
    }

    // ---- check ----
    main();
    assert_eq!(rest(&[7, 8, 9]), [8, 9]);
}

pub fn which_really_means() {
    fn rest<'a>(s: &'a [i32]) -> &'a [i32] {
        &s[1..]
    }

    // ---- check ----
    println!("rest(&[7, 8, 9]) = {:?}", rest(&[7, 8, 9]));
    assert_eq!(rest(&[7, 8, 9]), [8, 9]);
}

pub fn legal_a_static_array() {
    // ---- setup ----
    fn rest(s: &[i32]) -> &[i32] {
        &s[1..]
    }

    // ---- slide ----
    fn main() {
        let tail = {
            let arr = &[7, 8, 9];
            rest(arr)
        };
        println!("tail = {:?}", tail);
    }

    // ---- check ----
    main();
}

pub fn string_slices() {
    fn extension(file: &str) -> Option<&str> {
        // no new string: returns a slice of `file`
        file.rsplit_once('.').map(|(_, ext)| ext)
    }

    fn main() {
        if let Some(ext) = extension("notes.final.md") {
            println!("extension: {}", ext); // md
        } else {
            println!("no extension");
        }
    }

    // ---- check ----
    main();
    assert_eq!(extension("notes.final.md"), Some("md"));
    assert_eq!(extension("Makefile"), None);
}
