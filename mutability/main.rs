fn main() {
    let mut _immutable_binding = 1; // Make it mutable if you want to change it
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Now this is ok if _immutable_binding is mutable
    _immutable_binding += 1;
}
