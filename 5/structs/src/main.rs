struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method (this gets &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function (this doesn't get &self)
    fn square(size: u32) -> Rectangle {
        Rectangle { 
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("rust@gmail.com"),
        username: String::from("Rusty"),
        active: true,
        sign_in_count: 1,
    };

    let name: String = user1.username;
    user1.username = String::from("rrrust");

    let user2 = build_user(
        String::from("hello@gmail.com"),
        String::from("world"),
    );

    let user3 = User {
        email: String::from("aaa@gmail.com"),
        username: String::from("bbb"),
        ..user2
    };

    // Tuple Struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    // associated function (this doesn't connect to the struct)
    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);
    // in terminal
    // rect: Rectangle {
    //     width: 30,
    //     height: 50,
    // }

    println!("The area of the rectangle is {} square pixels.", rect.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
