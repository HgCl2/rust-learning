use std::io;

fn main() {
    let mut counter = 1;
    let right: &str = "The letter e";

    loop {
        println!("Riddle: I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Can't read line");

        let answer = answer.trim();
        if right == answer{
            println!("Number of trials: {counter}");
            break;
        }else{
            counter += 1;
        }
    }
    
}
