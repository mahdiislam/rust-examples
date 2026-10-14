//! § 12 Macros & panics
//!
//! Slides 12.3 and 12.4 panic on purpose. The runner catches the panic and
//! shows its message; `cargo test` checks that they do panic.

pub fn println_is_a_macro() {
    fn main() {
        println!("{}", "Hi from a macro!");
    }

    // ---- check ----
    main();
}

pub fn macros_expand_to_regular_code() {
    fn main() {
        use std::io::Write;
        std::io::stdout()
            .lock()
            .write_all(b"Hi from a macro!\n")
            .unwrap();
    }

    // ---- check ----
    main();
}

pub fn panic() {
    fn main() {
        panic!("Something broke");
    }
    // thread 'main' panicked at src/main.rs:2:5:
    // Something broke

    // ---- check ----
    main();
}

pub fn functions_that_panic() {
    fn main() {
        let some: Option<u8> = Some(42);
        some.unwrap(); // fine: 42

        let none: Option<u8> = None;
        none.unwrap(); // panics!
    }
    // called `Option::unwrap()` on a `None` value

    // ---- check ----
    main();
}
