fn main() {
    // variables immutable by default
    // if you assign it you can't change
    // value of variable
    let x = 5;

    // creates error
    // default values not given
    // if we run program compiler ask
    // type of variable
    let y;

    println!("The value of x is: {x}");

    // if you don't assign value of immutable variable
    // you can do it later
    y = 10;
    println!("The value of y is: {y}");
}
