//! § 21 Returning closures

pub fn returning_a_closure() {
    fn make_checker(secret: String) -> impl Fn(&str) -> bool {
        move |guess| guess == secret
    }

    fn main() {
        // `.into()` converts between types: here &'static str → String
        let check = make_checker("opensesame".into());
        println!("{}", check("abracadabra")); // false
        println!("{}", check("opensesame"));  // true
    }

    // ---- check ----
    main();
    let check = make_checker("opensesame".into());
    assert!(!check("abracadabra"));
    assert!(check("opensesame"));
}

pub fn capturing_a_borrowed_argument() {
    fn make_checker<'a>(secret: &'a str) -> impl Fn(&str) -> bool + 'a {
        move |guess| guess == secret
    }

    fn main() {
        let check = make_checker("opensesame");
        println!("{}", check("abracadabra")); // false
        println!("{}", check("opensesame"));  // true
    }

    // ---- check ----
    main();
    let check = make_checker("opensesame");
    assert!(!check("abracadabra"));
    assert!(check("opensesame"));
}

pub fn with_elided_lifetimes() {
    fn make_checker(secret: &str) -> impl Fn(&str) -> bool + '_ {
        move |guess| guess == secret
    }

    // ---- check ----
    let secret = String::from("opensesame");
    let check = make_checker(&secret);
    println!("{} {}", check("abracadabra"), check("opensesame"));
    assert!(check("opensesame"));
}
