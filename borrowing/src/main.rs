fn main() {
    let mut s = String::from("hello");

    // s borrows data to a function
    let len = calculate_length(&s);

    // if we make mutable reference
    // we can change text
    // but can exist only one mut reference
    change(&mut s);

    // few references of immutable var may exist
    let r1 = &s;
    let r2 = &s;
    // but immutable and mutable references
    // may not exist in one time
    // let r3 = &mut s;
    
    // reference's scope starts from where it is introduced
    // and continues through the last time reference is used
    println!("{} and {}", r1, r2);

    // after that we can make mutable reference
    let r3 = &mut s;
}

fn change(some_string: &mut String) {
    // we can change data using mutable reference
    // some_string is not owner
    some_string.push_str(", world!");

    // variable don't dropes
}

fn calculate_length(str: &String) -> usize {
    // when str tries read data it goes to s address
    // but not become owner of data
    return str.len();

    // variable don't dropes
}
