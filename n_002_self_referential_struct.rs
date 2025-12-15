struct Bad<'a> {
    s: String,
    r: &'a str,
}

pub fn run() {
    println!("Self Referential Struct");

    let mut bad = Bad {
        r: "",
        s: String::from("hello world"),
    };

    // Because of this line borrow occured
    // bad.r = bad.s.as_str();

    let bad2 = bad;

    println!("Bad r: {}, s: {}", bad2.r, bad2.s);
    println!("-----------------------");
}
