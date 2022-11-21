
// Regular stuct creation
// Using owned string types instead of references to strings allows 
// the struct to own all the data. &str can be used but then the struct
// does not own the string.
struct User
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() 
{
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 431,
    };

    let (email, username, active, sign_ins) = (user1.email, user1.username, user1.active, user1.sign_in_count);

    println!("{email}");
    println!("{username}");
    println!("{active}");
    println!("{sign_ins}");

    let black = Color(0,0,0);

    let origin = Point(0,0,0);

    let rect = Rectangle {
        width: 10,
        height: 5,
    };

    let rect1 = Rectangle {
        width: 15,
        height: 12,
    };

    let rect2 = Rectangle {
        width: 2, 
        height: 2,
    };

    println!("The area of the rectangle is: {}",
            area(&rect));

    println!("{:?}", &rect);

    println!("The area of the rectangle is: {}",
            rect.area());

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Cand rect hold rect2? {}", rect.can_hold(&rect2));
}

// Example of struct use
#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 
    {
        return self.width * self.height;
    }

    fn can_hold(&self, rect_cmp: &Rectangle) -> bool
    {
        return self.area() > rect_cmp.area();
    }
}

fn area(rect: &Rectangle) -> u32
{
    return rect.width * rect.height;
}