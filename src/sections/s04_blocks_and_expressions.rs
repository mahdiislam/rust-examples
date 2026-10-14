//! § 04 Blocks & expressions

pub fn blocks_have_their_own_scope() {
    // prints "inner", then "outer"
    fn main() {
        let label = "outer";
        {
            // a different `label`, local to this block
            let label = "inner";
            println!("{}", label);
        }
        println!("{}", label);
    }

    // ---- check ----
    main();
}

pub fn blocks_are_expressions() {
    let n = 5;

    // same thing:
    let n = { 5 };

    // ---- check ----
    println!("n = {n}");
    assert_eq!(n, 5);
}

pub fn the_tail_of_a_block() {
    let area = {
        let w = 3;  // statement
        let h = 4;  // statement
        w * h       // tail: the value of the whole block
    };

    // ---- check ----
    println!("area = {area}");
    assert_eq!(area, 12);
}

// The two functions have the same name, so each sits in its own block here.
pub fn implicit_return() {
    {
        fn pick_number() -> u32 {
            return 7;
        }

        // ---- check ----
        assert_eq!(pick_number(), 7);
    }

    // equivalent:
    {
        fn pick_number() -> u32 {
            7
        }

        // ---- check ----
        assert_eq!(pick_number(), 7);
        println!("both versions return {}", pick_number());
    }
}

pub fn if_is_an_expression() {
    fn pick_number(lucky: bool) -> u32 {
        if lucky { 7 } else { 3 }
    }

    // ---- check ----
    println!("lucky: {}, unlucky: {}", pick_number(true), pick_number(false));
    assert_eq!((pick_number(true), pick_number(false)), (7, 3));
}

pub fn match_is_an_expression() {
    fn pick_number(lucky: bool) -> u32 {
        match lucky {
            true => 7,
            false => 3,
        }
    }

    // ---- check ----
    println!("lucky: {}, unlucky: {}", pick_number(true), pick_number(false));
    assert_eq!((pick_number(true), pick_number(false)), (7, 3));
}
