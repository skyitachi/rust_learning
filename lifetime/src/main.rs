fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    let string1 = String::from("hello world");
    let string2 = "XYZ";
    println!("longest: {}", longest(&string1, string2));

    {
        let string2 = String::from("xyadfadsf");
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }
    // let res: &str;

    // {
    //     let string3 = String::from("xyadfadsf899111");
    //     res = longest(string1.as_str(), string3.as_str());
    // }
    // println!("The longest string is {}", res);

    let novel = String::from("it is a novel.");
    let first_sentence = novel.split('.')
        .next()
        .expect("could not find '.'");
    let i = ImportantExercerpt { part: first_sentence };
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest2<'a>(s1: &'a str, s2: &str) -> &'a str {
    s1
}

// fn longest3<'a>(s1: &str, s2: &str) -> &'a str {
//     let result = String::from("result");
//     result.as_str()
// }

struct ImportantExercerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExercerpt<'a> {
    fn third_rule(&self, y: &str) -> &str {
        self.part
    }
}