fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    {
        let s1 = String::from("hello");
        let s2 = s1;
        println!("{}", s2);
        // println!("{}", s1); // s1 has moved to s2
    }
    take_ownership(s);
    // println!("original s is: {}", s); // s has moved to take_ownership

    let x = 5;
    make_copy(x);
    println!("just copy: {}", x);
    
    {
        let s1 = gives_ownership();
        println!("s1 is: {}", s1);
        let s2 = String::from("hello world");
        println!("s2 is: {}", s2);
        let s3 = takes_and_give_ownership(s2);
        println!("s3 is: {}", s3);
    }

    {
        let s1 = String::from("hello world");
        let (s2, len) = calculate_length(s1);
        println!("s2 {} length is {}", s2, len);
    }
    // reference
    {
        let s1 = String::from("hello world");
        println!("s1 {} length is {}", s1, calculate_length_ref(&s1))
    }

    {
        let mut s1 = String::from("hello");
        println!("s1 is {}", s1);
        change(&mut s1);
        println!("s1 is {}", s1);
    }

    {
        let mut s1 = String::from("hello world");
        let s2 = first_word(&s1);
        println!("s2 is {}", s2);
        // s1.clear();
        // println!("s2 is {}", s2);
    }
}

fn take_ownership(s: String) {
    println!("take ownership: {}", s);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_give_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
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