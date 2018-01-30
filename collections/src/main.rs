fn main() {
    // vectors
    let v = vec![10, 2, 3];
    for i in &v {
        println!("{}", i);
    }
    let v0 = &v[0];
    println!("v0 {}", v0);

    // mutable vector
    let mut v = Vec::new();
    v.push(10);
    v.push(11);
    for i in &mut v {
        *i += 10;
    }
    for i in &mut v {
        println!("mut v: {}", i);
    }

    // generic vectors
    enum CommonValue {
        VFloat(f64),
        VInt(i32),
        VStr(String),
    }
    let cv = vec![
        CommonValue::VFloat(6.3),
        CommonValue::VInt(32),
        CommonValue::VStr(String::from("Hello world"))
    ];

    // String and &str
    let s1: &str = "123123";
    let s2: &str = "123";
    println!("s1 len: {}", s1.len());
    if s1 == s2 {
        println!("equal");
    } else {
        println!("not equal");
    }
    
    // Hash Map
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    // scores.insert(String::from("Yellow"), 20.0);

    println!("{:?}", scores);
    // 
    scores.entry(String::from("Blue")).or_insert(100);
    println!("{:?}", scores);

    let field_name = String::from("fc");
    let field_value = String::from("bl");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);   

    // update hashmap
    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // exercises
    let v1 = vec![1, 2, 3, 4];
    println!("v1 mean: {}", mean(&v1));
}


fn mean(input: &Vec<i32>) -> f64 {
    if input.len() == 0 {
        return -1.0;
    }
    let mut sum = 0.0;
    for i in input {
        sum += *i as f64;
    }
    return sum / (input.len() as f64);
}