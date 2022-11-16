fn main() {
    // data allocated in heap
    // owner is s1
    let s1 = String::from("Hello");
    // data moved to s2
    // now owner is s2
    let s2 = s1;
    // now owner is s3
    let s3 = s2;

    // can't accessing data from s1 and s2
    // because owner only s3
    // s2 and s1 will be deleted when scope they 
    // goes out from current scope
    println!("{}", s3); 

    // compiler will free memory(drop function)
    // only s3(owner), and delete s1 and s2 as pointers(void)
}
