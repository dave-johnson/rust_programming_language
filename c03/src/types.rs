pub fn some_types() {
    // Integer
    let f: u8 = 255;
    let tup: (&str, i32) = ("some string", 100);
    let (msg: &str, count: i32) = tup;

    println!("msg: {} with count: {}", msg, count);

    let error_codes: [i32; _] = [200, 404, 500];
    let not_found: i32 = error_codes[1];

    let  byte: [i32; _] = [0; 8]; // 8 elements filled with 0
    
}
