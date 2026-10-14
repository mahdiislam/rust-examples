//! § 15 'static & ownership
//!
//! Slide 15.2 "A String is not 'static" is in `does_not_compile/`.
//! Slide 15.6 "Owned vs. reference types" has bullet points only; its text is in the catalog.

pub fn the_static_lifetime() {
    struct Pet {
        name: &'static str,
    }

    fn main() {
        let p = Pet { name: "Ferris" };
    }

    // ---- check ----
    main();
    let p = Pet { name: "Ferris" };
    println!("p.name = {}", p.name);
}

pub fn fix_a_be_generic_over_a_lifetime() {
    struct Pet<'a> {
        name: &'a str,
    }

    fn main() {
        let name = format!("Fer{}", "ris");
        let p = Pet { name: &name };
        // `p` cannot outlive `name`
    }

    // ---- check ----
    main();
    let name = format!("Fer{}", "ris");
    println!("p.name = {}", Pet { name: &name }.name);
}

pub fn fix_b_take_ownership() {
    struct Pet {
        name: String,
    }

    fn main() {
        let name = format!("Fer{}", "ris");
        let p = Pet { name: name };
        // `name` moved into `p`: their lifetimes are no longer tied
    }

    // ---- check ----
    main();
    let p = Pet { name: format!("Fer{}", "ris") };
    println!("p.name = {}", p.name);
}

// `name` is moved by the first line, so each line gets a fresh `name` in its own block.
pub fn field_init_shorthand() {
    // ---- setup ----
    struct Pet {
        name: String,
    }

    {
        // ---- setup ----
        let name = String::from("Ferris");

        // ---- slide ----
        let p = Pet { name: name };

        // ---- check ----
        println!("long form: {}", p.name);
    }

    // shorthand:
    {
        // ---- setup ----
        let name = String::from("Ferris");

        // ---- slide ----
        let p = Pet { name };

        // ---- check ----
        println!("shorthand: {}", p.name);
    }
}
