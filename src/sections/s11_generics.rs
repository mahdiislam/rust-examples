//! § 11 Generics

pub fn generic_functions() {
    fn wrap<T>(item: T) {
        // do something with `item`
    }

    // ---- check ----
    wrap(1);
    wrap("one");
    println!("wrap() accepted an i32 and a &str");
}

pub fn several_type_parameters() {
    fn pair_up<A, B>(first: A, second: B) {
        // do something with `first` and `second`
    }

    // ---- check ----
    pair_up(1, 'b');
    println!("pair_up() accepted an i32 and a char");
}

pub fn trait_bounds() {
    use std::fmt::{Debug, Display};

    fn show<T: Display>(item: T) {
        println!("item = {}", item);
    }

    fn inspect<T: Debug>(item: T) {
        println!("item = {:?}", item);
    }

    // ---- check ----
    show(42);
    inspect(vec!["a", "b"]);
}

pub fn where_clauses() {
    // ---- setup ----
    use std::fmt::Display;

    // ---- slide ----
    fn show<T>(item: T)
    where
        T: Display,
    {
        println!("item = {}", item);
    }

    // ---- check ----
    show("hello");
}

pub fn multiple_bounds() {
    use std::fmt::Debug;

    fn same<T>(a: T, b: T)
    where
        T: Debug + PartialEq,
    {
        let op = if a == b { "==" } else { "!=" };
        println!("{:?} {} {:?}", a, op, b);
    }

    fn main() {
        same(3, 4); // prints: 3 != 4
    }

    // ---- check ----
    main();
}

pub fn turbofish() {
    use std::any::type_name;

    fn main() {
        println!("{}", type_name::<u8>());           // "u8"
        println!("{}", type_name::<(bool, &str)>()); // "(bool, &str)"
    }

    // ---- check ----
    main();
    assert_eq!(type_name::<u8>(), "u8");
    assert_eq!(type_name::<(bool, &str)>(), "(bool, &str)");
}

pub fn generic_structs() {
    struct Both<T> {
        left: T,
        right: T,
    }

    fn name_of<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    fn main() {
        let a = Both { left: 1.5, right: 2.5 };
        let b = Both { left: 'x', right: 'y' };
        println!("{}", name_of(&a)); // "my_crate::Both<f64>"
        println!("{}", name_of(&b)); // "my_crate::Both<char>"
    }

    // ---- check ----
    main();
    // The full name starts with this crate's module path instead of `my_crate`.
    assert!(name_of(&Both { left: 1.5, right: 2.5 }).ends_with("::Both<f64>"));
    assert!(name_of(&Both { left: 'x', right: 'y' }).ends_with("::Both<char>"));
}

pub fn vec_t_is_generic() {
    // ---- setup ----
    fn name_of<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    // ---- slide ----
    fn main() {
        let mut nums = Vec::new();
        nums.push(10);
        let mut flags = Vec::new();
        flags.push(true);
        println!("{}", name_of(&nums));  // "alloc::vec::Vec<i32>"
        println!("{}", name_of(&flags)); // "alloc::vec::Vec<bool>"
    }

    // ---- check ----
    main();
    assert_eq!(name_of(&vec![10]), "alloc::vec::Vec<i32>");
    assert_eq!(name_of(&vec![true]), "alloc::vec::Vec<bool>");
}

pub fn vec_literals() {
    // ---- setup ----
    fn name_of<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    // ---- slide ----
    fn main() {
        let nums = vec![10, 20, 30];
        let flags = vec![true, true, false];
        println!("{}", name_of(&nums));  // "alloc::vec::Vec<i32>"
        println!("{}", name_of(&flags)); // "alloc::vec::Vec<bool>"
    }

    // ---- check ----
    main();
}
