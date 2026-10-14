//! § 19 Closures
//!
//! Slides 19.3, 19.6, 19.9, 19.11 and 19.12 are compile errors and live in
//! `does_not_compile/`. Slide 19.15 panics on purpose.

pub fn closures() {
    fn for_each_day<F>(f: F)
    where
        F: Fn(&'static str),
    {
        f("Mon");
        f("Tue");
        f("Wed");
    }

    fn main() {
        for_each_day(|day| println!("Good morning, {}", day));
    }
    // Good morning, Mon / Tue / Wed

    // ---- check ----
    main();
}

pub fn closures_borrow() {
    // ---- setup ----
    fn for_each_day<F>(f: F)
    where
        F: Fn(&'static str),
    {
        f("Mon");
        f("Tue");
        f("Wed");
    }

    // ---- slide ----
    fn main() {
        let greeting = String::from("Have a nice");
        for_each_day(|day| println!("{} {}", greeting, day));
        // the closure borrows `greeting`, so it can't outlive it
    }

    // ---- check ----
    main();
}

pub fn move_closures() {
    // ---- setup: the 'static version from slide 19.3 ----
    fn for_each_day<F>(f: F)
    where
        F: Fn(&'static str) + 'static, // `F` must now be 'static
    {
        f("Mon");
    }

    // ---- slide ----
    fn main() {
        let greeting = String::from("Have a nice");
        for_each_day(move |day| println!("{} {}", greeting, day));
        // `greeting` is moved into the closure, not borrowed
    }

    // ---- check ----
    main();
}

pub fn fn_can_be_called_freely() {
    fn apply<F>(f: F)
    where
        F: Fn(i32) -> i32,
    {
        println!("{}", f(f(3)));
    }

    fn main() {
        apply(|x| x + 1);
    }
    // prints 5

    // ---- check ----
    main();
    let f = |x: i32| x + 1;
    assert_eq!(f(f(3)), 5);
}

pub fn fnmut_sequential_calls_are_fine() {
    fn apply<F>(mut f: F)
    where
        F: FnMut(i32) -> i32,
    {
        let first = f(3);
        println!("{}", f(first));
    }

    fn main() {
        apply(|x| x + 1);
    }
    // prints 5

    // ---- check ----
    main();
}

pub fn why_fnmut_exists() {
    // ---- setup ----
    fn apply<F>(mut f: F)
    where
        F: FnMut(i32) -> i32,
    {
        let first = f(3);
        println!("{}", f(first));
    }

    // ---- slide ----
    fn main() {
        let mut calls = 0;
        apply(|x| {
            calls += 1;
            x * calls
        });
    }
    // 3 * 1 = 3, then 3 * 2 = 6: prints 6

    // ---- check ----
    main();
    let mut calls = 0;
    let mut f = |x: i32| {
        calls += 1;
        x * calls
    };
    let first = f(3);
    assert_eq!(f(first), 6);
}

pub fn fnonce() {
    fn run_once<F>(f: F)
    where
        F: FnOnce() -> String,
    {
        println!("{}", f());
    }

    fn main() {
        let msg = String::from("done");
        run_once(move || msg);
        // `msg` moves into the closure, then out through its return value
    }

    // ---- check ----
    main();
}

pub fn cloning_instead() {
    // ---- setup ----
    fn run_once<F>(f: F)
    where
        F: FnOnce() -> String,
    {
        println!("{}", f());
    }

    // ---- slide ----
    fn main() {
        let msg = String::from("done");
        run_once(|| msg.clone());
        run_once(|| msg.clone());
    }

    // ---- check ----
    main();
}

pub fn closures_with_two_arguments() {
    fn order<F>(a: i32, b: i32, before: F)
    where
        F: Fn(i32, i32) -> bool,
    {
        let (first, second) = if before(a, b) { (a, b) } else { (b, a) };
        println!("{} comes before {}", first, second);
    }

    fn main() {
        order(8, 2, |x, y| x < y); // 2 comes before 8
    }

    // ---- check ----
    main();
}

pub fn ignoring_arguments() {
    // ---- setup ----
    fn order<F>(a: i32, b: i32, before: F)
    where
        F: Fn(i32, i32) -> bool,
    {
        let (first, second) = if before(a, b) { (a, b) } else { (b, a) };
        println!("{} comes before {}", first, second);
    }

    // ---- slide ----
    fn main() {
        order(8, 2, |_, _| panic!("No ordering today!"));
    }

    // ---- check ----
    main();
}

pub fn a_closure_in_a_loop() {
    fn countdown<F>(from: u32, tick: F)
    where
        F: Fn(u32),
    {
        for i in (1..=from).rev() {
            tick(i);
        }
    }

    fn main() {
        countdown(3, |i| println!("{}...", i));
    }
    // 3...  2...  1...

    // ---- check ----
    main();
}

pub fn the_toilet_closure() {
    // ---- setup ----
    fn countdown<F>(from: u32, tick: F)
    where
        F: Fn(u32),
    {
        for i in (1..=from).rev() {
            tick(i);
        }
    }

    // ---- slide ----
    fn main() {
        countdown(3, |_| ());
    }

    // ---- check ----
    main();
    println!("(the toilet closure printed nothing, as it should)");
}
