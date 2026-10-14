//! § 20 Loops & iterators

pub fn for_over_a_vec() {
    fn main() {
        for n in vec![4, 8, 15] {
            println!("Lucky number {}", n);
        }
    }

    // ---- check ----
    main();
}

pub fn for_over_a_slice() {
    fn main() {
        for n in &[4, 8, 15] {
            println!("Lucky number {}", n);
        }
    }
    // Lucky number 4 / 8 / 15

    // ---- check ----
    main();
}

pub fn for_over_an_iterator() {
    fn main() {
        for c in "crab".chars() {
            println!("Letter: {}", c);
        }
    }
    // Letter: c / r / a / b

    // ---- check ----
    main();
}

pub fn iterator_adapters() {
    fn main() {
        for c in "rUsT iS fUn"
            .chars()
            .filter(|c| c.is_uppercase())
            .flat_map(|c| c.to_lowercase())
        {
            print!("{}", c);
        }
        println!();
    }
    // prints: utsu

    // ---- check ----
    main();
    let collected: String = "rUsT iS fUn"
        .chars()
        .filter(|c| c.is_uppercase())
        .flat_map(|c| c.to_lowercase())
        .collect();
    assert_eq!(collected, "utsu");
}
