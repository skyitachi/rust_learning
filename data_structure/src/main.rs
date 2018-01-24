struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("skbysp@gmail.com"),
        username: String::from("skbysp"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String::from("skyitachi@gmail.com");

    let user2 = build_user(String::from("skbysp@gmail.com"), String::from("skbysp"));

    let user3 = User {
        email: String::from("yspzmx@163.com"),
        username: String::from("yspzmx"),
        ..user1
    };
    {
        let rect = (30, 50);
        println!("{}", area(rect));
    }
    {
        let rect = Rectangle { width: 30, height: 50 };
        println!("{}", area2(&rect));
        println!("rect is {:?}", rect);
        println!("rect is {:#?}", rect);
    }
    {
        let rect = Rectangle { width: 30, height: 50 };
        println!("area is {}", rect.area());
    } 
    {
        let rect = Rectangle::square(100);
        let rect2 = Rectangle { width: 30, height: 50 };
        println!("can_hold {}", rect.can_hold(&rect2));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}