// Slide 19.3 · ...which can conflict with 'static
//
// This program must NOT compile. rustc rejects it with:
//     error[E0373]: closure may outlive the current function, but it borrows `greeting`
// `cargo test` checks that this is the only error code rustc reports.

fn for_each_day<F>(f: F)
where
    F: Fn(&'static str) + 'static, // `F` must now be 'static
{
    f("Mon");
}

fn main() {
    let greeting = String::from("Have a nice");
    for_each_day(|day| println!("{} {}", greeting, day));
    // error: closure may outlive the current function,
    //        but it borrows `greeting`
}
