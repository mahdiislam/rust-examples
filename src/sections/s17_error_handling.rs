//! § 17 Error handling
//!
//! Slides 17.2 and 17.3 panic on purpose. The runner catches the panic and
//! shows its message; `cargo test` checks that they do panic.

pub fn fallible_functions_return_result() {
    fn main() {
        let ok = std::str::from_utf8(&[0x68, 0x69]);
        println!("{:?}", ok);  // Ok("hi")

        let bad = std::str::from_utf8(&[0xff]);
        println!("{:?}", bad); // Err(Utf8Error { valid_up_to: 0, error_len: Some(1) })
    }

    // ---- check ----
    main();
    assert_eq!(format!("{:?}", std::str::from_utf8(&[0x68, 0x69])), r#"Ok("hi")"#);
    assert_eq!(
        format!("{:?}", std::str::from_utf8(&[0xff])),
        "Err(Utf8Error { valid_up_to: 0, error_len: Some(1) })"
    );
}

pub fn unwrap() {
    fn main() {
        let s = std::str::from_utf8(&[0x68, 0x69]).unwrap();
        println!("{:?}", s); // "hi"

        let s = std::str::from_utf8(&[0xff]).unwrap();
        // panics: called `Result::unwrap()` on an `Err` value: Utf8Error { ... }
    }

    // ---- check ----
    main();
}

pub fn expect() {
    fn main() {
        let s = std::str::from_utf8(&[0xff]).expect("input must be UTF-8");
        // panics: input must be UTF-8: Utf8Error { ... }
    }

    // ---- check ----
    main();
}

pub fn match_on_a_result() {
    fn main() {
        match std::str::from_utf8(&[0x68, 0x69]) {
            Ok(s) => println!("{}", s),
            Err(e) => panic!("{}", e),
        }
        // prints: hi
    }

    // ---- check ----
    main();
}

pub fn if_let_on_a_result() {
    fn main() {
        if let Ok(s) = std::str::from_utf8(&[0x68, 0x69]) {
            println!("{}", s);
        }
        // prints: hi
    }

    // ---- check ----
    main();
}

pub fn bubbling_the_error_up() {
    fn main() -> Result<(), std::str::Utf8Error> {
        match std::str::from_utf8(&[0x68, 0x69]) {
            Ok(s) => println!("{}", s),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    // ---- check ----
    assert_eq!(main(), Ok(()));
}

pub fn the_question_mark_operator() {
    fn main() -> Result<(), std::str::Utf8Error> {
        let s = std::str::from_utf8(&[0x68, 0x69])?;
        println!("{}", s);
        Ok(())
    }

    // ---- check ----
    assert_eq!(main(), Ok(()));
}
