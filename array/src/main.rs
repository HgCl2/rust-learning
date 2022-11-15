// if you want allocate you data in stack use array
// vector is a similar collection type provided by std lib
// that is allowed to grow or shrink in size.
fn main() {
    // array have a fixed length
    // by default array is immutable
    let a = [1, 2, 3, 4, 5];

    //writing array's type and capacity
    let b: [i32; 5] = [2, 3, 4, 5, 6];

    // syntax for initialize an array with same value
    // array contain 3 value five times
    let mut c = [3;5];

    c[0] = 6;

    // only this way you can print first elem in array
    println!("{}", a[0]);
    println!("{}", b[0]);
    println!("{}", c[0]);
}
