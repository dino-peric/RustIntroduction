fn main() {
    let s = String::from("Hello");
    takes_ownership(s);

    let s2 = gives_ownership();

    let s3 = takes_and_gives_back(s2);

    println!("{s3}");

    let (s3, len) = calculate_length(s3);

    println!("The length of '{}' is {}.", s3, len);

    let s4 = String::from("Blabla");
    let s4_len = calculate_length_with_ref(&s4);

    println!("{s4_len}");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    // this doesn't work because we can't have two references to a mutable value at once
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn gives_ownership() -> String {
    let some_string = String::from("bla");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_with_ref(s: &String) -> usize {
    // references are immutable by default so this gives an error
    // s.push_str("ba");
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("blaBliBla")
}