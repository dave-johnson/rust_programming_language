fn main() {
    // ownership rules
    // 1. Each value in Rust has a variable that's called its owner
    // 2. there csn only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    let s1 = String::from("hello");
    // let s2 = s1.clone();
    let len_ref = calculate_length_ref(&String::from("dave was here"));
    println!("the length of '{}' is {}.", s1, len_ref);

    let (s2, len) = calculate_length(s1);  // s1 will get dropped
    println!("the length of '{}' is {}.", s2, len);

    // println!("{}, world", s1);

    let mut s4 = String::from("dave was here");
    change(&mut s4);
    println!("{}", s4);


    let s1 = String::from("who has control?");
    println!("{}", s1);
    println!("{}", s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(".  added some stuff");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();  // len() return the length of a string
    (s, length)
}
fn calculate_length_ref(s: &String) -> (usize) {
    let length = s.len();  // len() return the length of a string
    length
}
