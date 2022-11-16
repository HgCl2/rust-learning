fn main() {
    let mut s = String::from("hello");

    // s borrows data to a function
    let len = calculate_length(&s);

    // if we make mutable reference
    // we can change text
    // but can exist only one mut reference
    change(&mut s);
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
