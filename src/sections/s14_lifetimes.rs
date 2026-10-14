//! § 14 Lifetimes
//!
//! Slides 14.3, 14.5 and 14.6 are borrow-checker errors and live in `does_not_compile/`.

pub fn bindings_have_lifetimes() {
    fn main() {
        // `n` doesn't exist yet
        {
            let n = 5; // `n` starts existing
            println!("n = {}", n);
            // `n` stops existing
        }
        // `n` no longer exists
    }

    // ---- check ----
    main();
}

pub fn references_have_lifetimes() {
    fn main() {
        {
            let n = 5;      // `n` starts existing
            let r = &n;     // `r` starts existing, borrowing `n`
            println!("r = {}", r);
            // `r` stops existing, then `n` stops existing
        }
    }

    // ---- check ----
    main();
}

pub fn many_shared_borrows() {
    fn main() {
        let n = 5;
        let r1 = &n;
        let r2 = &n;
        let r3 = &n;
        println!("{} {} {}", r1, r2, r3);
    }

    // ---- check ----
    main();
}

pub fn lifetimes_in_function_arguments() {
    fn peek(v: &u32) {
        // `v` is borrowed from the caller
        // for as long as this function runs
    }

    // ---- check ----
    let n = 7;
    peek(&n);
    println!("still own n = {n} after lending it to peek()");
}

// Two functions named `peek` can't share a scope, so each gets its own block.
pub fn named_lifetimes() {
    {
        // elided (unnamed) lifetime:
        fn peek(v: &u32) {}

        // ---- check ----
        peek(&1);
    }
    {
        // named lifetime:
        fn peek<'a>(v: &'a u32) {}

        // ---- check ----
        peek(&1);
    }
    println!("both versions of peek() accept the same arguments");
}

pub fn returning_a_borrowed_reference() {
    struct Reading {
        celsius: i32,
    }

    fn celsius_of<'a>(r: &'a Reading) -> &'a i32 {
        &r.celsius
    }

    fn main() {
        let r = Reading { celsius: 18 };
        let c = celsius_of(&r);
        // `c` borrows `r`: it can't outlive it, and while `c` lives,
        // `r` can't be mutated, moved or mutably borrowed
    }

    // ---- check ----
    main();
    let r = Reading { celsius: 18 };
    println!("celsius_of(&r) = {}", celsius_of(&r));
    assert_eq!(*celsius_of(&r), 18);
}

pub fn lifetime_elision() {
    // ---- setup ----
    struct Reading {
        celsius: i32,
    }
    let r = Reading { celsius: 18 };

    // ---- slide ----
    {
        fn celsius_of<'a>(r: &'a Reading) -> &'a i32 {
            &r.celsius
        }

        // ---- check ----
        assert_eq!(*celsius_of(&r), 18);
    }

    // equivalent:
    {
        fn celsius_of(r: &Reading) -> &i32 {
            &r.celsius
        }

        // ---- check ----
        assert_eq!(*celsius_of(&r), 18);
        println!("both versions return {}", celsius_of(&r));
    }
}

pub fn structs_generic_over_lifetimes() {
    struct Borrowed<'a> {
        inner: &'a u32,
    }

    fn main() {
        let n: u32 = 64;
        let b = Borrowed { inner: &n };
        // `b` cannot outlive `n`
    }

    // ---- check ----
    main();
    let n: u32 = 64;
    let b = Borrowed { inner: &n };
    println!("b.inner = {}", b.inner);
}

pub fn built_by_a_function() {
    // ---- setup ----
    struct Borrowed<'a> {
        inner: &'a u32,
    }

    // ---- slide ----
    fn wrap<'a>(n: &'a u32) -> Borrowed<'a> {
        Borrowed { inner: n }
    }

    fn main() {
        let n: u32 = 64;
        let b = wrap(&n);
        // `b` cannot outlive `n`
    }

    // ---- check ----
    main();
    assert_eq!(*wrap(&64).inner, 64);
}

pub fn with_an_elided_lifetime() {
    // ---- setup ----
    struct Borrowed<'a> {
        inner: &'a u32,
    }

    // ---- slide ----
    fn wrap(n: &u32) -> Borrowed<'_> {
        Borrowed { inner: n }
    }

    // ---- check ----
    let n = 64;
    println!("wrap(&n).inner = {}", wrap(&n).inner);
}

pub fn impl_blocks_generic_over_lifetimes() {
    // ---- setup ----
    struct Borrowed<'a> {
        inner: &'a u32,
    }

    // ---- slide ----
    impl<'a> Borrowed<'a> {
        fn get(&'a self) -> &'a u32 {
            self.inner
        }
    }

    fn main() {
        let n: u32 = 64;
        let b = Borrowed { inner: &n };
        let r = b.get();
        // neither `b` nor `r` can outlive `n`
    }

    // ---- check ----
    main();
    let n: u32 = 64;
    assert_eq!(*Borrowed { inner: &n }.get(), 64);
}

pub fn eliding_in_methods() {
    // ---- setup ----
    struct Borrowed<'a> {
        inner: &'a u32,
    }

    // ---- slide ----
    impl<'a> Borrowed<'a> {
        fn get(&self) -> &u32 {
            self.inner
        }
    }

    // ---- check ----
    let n: u32 = 64;
    println!("get() = {}", Borrowed { inner: &n }.get());
}

pub fn eliding_even_harder() {
    // ---- setup ----
    struct Borrowed<'a> {
        inner: &'a u32,
    }

    // ---- slide ----
    impl Borrowed<'_> {
        fn get(&self) -> &u32 {
            self.inner
        }
    }

    // ---- check ----
    let n: u32 = 64;
    println!("get() = {}", Borrowed { inner: &n }.get());
}
