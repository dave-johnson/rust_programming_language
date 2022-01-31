


fn main() {
    // let mut x: i32 = 5;
    // println!("the number is {}", x);
    // x = 3;
    // println!("had to mut the number is {}", x);
    // const BIG_NUMBER: u32 = 100_000;
    // println!("{}", BIG_NUMBER);

    // some_types();
    println!("get_back = {:?}", get_back());
    println!("get_back_more = {:?}", get_back_more());
}

fn get_back() -> i32 {
    return 42;
}
fn get_back_more() -> (i32, i32, &'static str) {
    return (42, 64, "some object");
}

 fn some_types() {
    // Integer
    // let f: u8 = 255;
    let tup: (&str, i32) = ("some string", 100);
    let (msg2, count) = tup;

    println!("msg: {} with count: {}", msg2, count);

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    println!(" errors: {:?} with not_found {}", error_codes, not_found);

    let byte = [0; 8]; // 8 elements filled with 0
    // let byte = [0; 8];
    println!("byte {:?}", byte);
}