fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let user3 = build_user(String::from("example3@example.com"), String::from("example3"));

    user1.active = false;
    println!("{}, {}, {}, {}", user1.username, user1.email, user1.active, user1.sign_in_count);
    println!("{}, {}, {}, {}", user2.username, user2.email, user2.active, user2.sign_in_count);
    println!("{}, {}, {}, {}", user3.username, user3.email, user3.active, user3.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0 == origin.0);

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rec1));
    println!("{:#?}", rec1);
    println!("{}", rec1.area());

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));

    let square = Rectangle::square(32);
    println!("{:#?}", square);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // same name
        username, // same name
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}