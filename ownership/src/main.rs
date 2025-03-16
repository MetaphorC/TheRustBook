//fn main() {
//    let s = "hello";  // s is valid from here
//
//    // do stuff with s
//}   // scope for s ended
    //
    //
    // 1. each value has an owner
    // 2. there can be only one owner for a value at a time
    // 3. when the owner goes out of scope, the value will be dropped


fn main() {
    let mut s = String::from("Holy ");
    s.push_str("MOLY!"); // push_str() appends the string literal
    println!("{s}") // this will print Holy Moly
}
