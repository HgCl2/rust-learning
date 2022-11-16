fn main() {
    // data allocated in heap
    // owner is s1
    let s1 = String::from("Hello");
    // data moved to s2
    // now owner is s2
    let s2 = s1;
    
    // data moved to function and returns
    // s3 is owner
    let s3 = takes_and_gives_back(s2);


    // can't accessing data from s1 and s2
    // because owner only s3
    // s2 and s1 will be deleted when scope they 
    // goes out from current scope
    println!("{}", s3); 

    // compiler will free memory(drop function)
    // only s3(owner), and delete s1 and s2 as pointers(void)
}


fn takes_and_gives_back(a_string: String) -> String {
    // a_sting comes into scope
    //and returns from it
    return a_string;
}
