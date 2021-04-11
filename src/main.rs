// references are immutable by default

fn main () {
    let mut s = String::from ("Hello");
    let len = calculate_length (&s);
    println! ("{}", len);

    // change(&s); compile-time error

    // pass a mutable reference
    change_1 (&mut s);
    println! ("{}", s);

    valid ();

    // invalid_1 ();
    // invalid_2 ();
}

// mark s as a reference
fn calculate_length (s : &String) -> usize {
    s.len ()
    // s goes out of scope but nothing happens since the function doesn't own it
}

// mark s as mutable
fn change_1 (s : &mut String) {
    // we can modify the reference
    s.push_str (" Woo!");
}

// fn change (s : &String) {
//     // cannot modify a reference since we're "borrowing" the value
//     s.push_str(", woo!");
// }

// fn invalid_1 () {
//     let s = String::new ();
//     let r1 = &mut s;
//     let r2 = &mut s;
//     // compile-time error : cannot have two mutable borrows in the same scope
//     println! ("{} {}", r1, r2);
// }
//
// fn invalid_2 () {
//     let s = String::new();
//     let r1 = &s;
//     let r2 = &mut s;
//     // compile-time error : cannot have a mutable borrow in the same scope as a reference
//     println! ("{} {}", r1, r2)
// }

fn valid () {
    let mut s = String::from ("Valid");
    let r1 = &s;
    println! ("{}", r1);
    let r2 = &mut s;
    // this is okay because r1 went out of scope after the first println
    println! ("{}", r2);
}