const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("constant {}", MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let space = " ";
    let space = space.len();
    println!("space length is: {}", space);

    // compile error
    // let guess = "42".parse().expect("");
    
    let tup = (500, 64, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x2 = (500, 64, 1);
    let _five_hundred = x2.0;
    let _sixty_four = x2.1;
    let _one = x2.2;

    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // [3, 3, 3, 3, 3];

    // panic
    /*
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is {}", element);
    */

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x, y is {}, {}", x, y);

    println!("The value of five() is {}", five());

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is {}", result);

    let array = [1, 2, 3, 4, 5];
    for element in array.iter() {
        println!("The value is {}", element);
    }

    for number in (1..4).rev() {
        println!("The number is: {}", number);
    }
}

fn five() -> i32 {
    5
}

// compilation error
/*
fn add() -> i32 {
    x + 1; <<
}
*/