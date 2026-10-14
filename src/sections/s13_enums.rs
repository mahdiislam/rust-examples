//! § 13 Enums
//!
//! These two slides define their own `Option` and `Result`. Inside each
//! function, the local enum hides the standard one.

pub fn option_is_an_enum() {
    enum Option<T> {
        None,
        Some(T),
    }

    impl<T> Option<T> {
        fn unwrap(self) -> T {
            // enum variants work in patterns
            match self {
                Self::Some(v) => v,
                Self::None => panic!("unwrap called on None"),
            }
        }
    }

    // ---- check ----
    let some: Option<i32> = Option::Some(128);
    let value = some.unwrap();
    println!("Option::Some(128).unwrap() = {value}");
    assert_eq!(value, 128);
    // `Option::None.unwrap()` would panic with "unwrap called on None",
    // just like the standard Option does (slide 12.4).
}

pub fn result_is_an_enum_too() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // ---- check ----
    let results: [Result<u8, &str>; 2] = [Result::Ok(1), Result::Err("disk full")];
    for result in results {
        match result {
            Result::Ok(value) => println!("Ok({value})"),
            Result::Err(error) => println!("Err({error:?})"),
        }
    }
}
