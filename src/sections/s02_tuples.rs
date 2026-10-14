//! § 02 Tuples

pub fn tuples() {
    let point = (3.5, 'z');
    point.0; // 3.5
    point.1; // 'z'

    // ---- check ----
    println!("point = {point:?}");
    assert_eq!(point.0, 3.5);
    assert_eq!(point.1, 'z');
}

pub fn annotating_a_tuple() {
    let point: (f64, char) = (3.5, 'z');

    // ---- check ----
    println!("point = {point:?}");
}

pub fn destructuring_a_tuple() {
    let (dist, label) = (3.5, 'z');
    // `dist` is 3.5, `label` is 'z'

    // ---- check ----
    println!("dist = {dist}, label = {label:?}");
    assert_eq!((dist, label), (3.5, 'z'));
}

pub fn destructuring_a_returned_tuple() {
    // ---- setup ----
    let items = [10, 20, 30, 40];
    let mid = 1;

    // ---- slide ----
    let (head, tail) = items.split_at(mid);

    // ---- check ----
    println!("head = {head:?}, tail = {tail:?}");
    assert_eq!(head, [10]);
    assert_eq!(tail, [20, 30, 40]);
}

pub fn ignoring_part_of_a_tuple() {
    // ---- setup ----
    let items = [10, 20, 30, 40];
    let mid = 1;

    // ---- slide ----
    let (_, tail) = items.split_at(mid);

    // ---- check ----
    println!("tail = {tail:?}");
    assert_eq!(tail, [20, 30, 40]);
}
