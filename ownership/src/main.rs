fn main() {
    /*
    let s1 = String::from("hello");
    let s2 = s1;

    // compilation error. s1's ownership transferred to s2
    println!("{}", s1);
    */

    // move
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // stack only data
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s);
    // takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    let s2 = String::from("hello");
    println!("s2 = {}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);

    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    // immutable references dropped
    println!("{}, {}", s1, s2);

    let s3 = &mut s;
    println!("{}", s3);

    let mut s = String::from("hello world");
    let word = first_word_length(&s);
    println!("{}", word);
    s.clear();

    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{}, {}", hello, world);
    
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // having this after clear causes compilation error
    println!("the first word is: {}", word);
    s.clear();
    
    // string literals are string slices
}

// drop called. memory freed
fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}