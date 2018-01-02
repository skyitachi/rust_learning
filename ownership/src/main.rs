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
}

fn take_ownership(s: String) {
    println!("take ownership: {}", s);
}
