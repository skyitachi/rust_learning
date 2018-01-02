fn main() {
    let mut x = 5;
    println!("Hello, world!");
    println!("x is {}", x);
    x = 10;
    println!("x is {}", x);
    let y = 100_0000;
    println!("y is {}", y);
    let len: u32 = "42".parse().expect("Not a Number");
    println!("len is {}", len);
    // data_types

    {
        let _x = 4.2;
        let z = 0.0;
        println!("z is {}", z);
    }
    // compound data types
    {
        let tup: (bool, f64, u8) = (true, 10.1, 2);
        let (x, y, z) = tup;
        println!("x is {}", x);
        let a = tup.2;
        println!("a is {}", a);
    }
    another_function(10);
    {
        let con = true;
        let v = if con {
            10
        } else {
            0
        };
        println!("v is {}", v);

    }
    {
        let a = [1, 2, 3, 4, 5];
        for ele in a.iter() {
            println!("the value is {}", ele);
        }
    }
}

fn another_function(x: u32) -> u32 {
    println!("param x is {}", x);
    println!("another function called");
    10
}