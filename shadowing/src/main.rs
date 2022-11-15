fn main() {
    let x = 5;
    
    // x until immutable
    // but now his value is 6
    let x = x + 1;

    {
        // value of x is 12 in inner scope
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x in external scope is: {x}");
}
