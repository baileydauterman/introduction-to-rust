use std::fmt;
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32,
}

# [derive(Debug, PartialEq)]
struct EvenNumber(i32);

struct Circle {
    radius: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    } 
}

fn main() {
    // From and Into 
    // The Form and Into traits are inherentley linked, and this is actually part of its
    // implementation. If you are able to convert type A from type B, then it should be 
    // easy to convert type B to type A
    //

    // FROM

    // This trait allows for a type to define how to create iself form another type, hence
    // providing a very simple mechanism for converting between several types. There are numerous
    // implementations fo this trait within the standard library for conversion of primative and
    // common types
    
    // convert str to string 
    let _my_str = "hello";
    let _my_string = String::from(_my_str);

    // Creating custom conversions
    let num = Number::from(45);
    println!("My number is {:?}", num);

    // If we do not specify the type of num here we will receive this error
    //
    //error[E0282]: type annotations needed
    //  --> src/main.rs:37:9
    //   |
    //37 |     let num = int.into();
    //   |         ^^^
    //   |
    //help: consider giving `num` an explicit type
    //   |
    //37 |     let num: _ = int.into();
    //   |            +++
    //
    //For more information about this error, try `rustc --explain E0282`.
    //error: could not compile `conversion` due to previous error 
    let int = 6;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // TryFrom and TryInto
    // These traits are used to be fallible conversion and return Results
    //

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(13), Err(()));

    // TryInto 

    let result: Result<EvenNumber, ()> = 8i32.try_into(); 
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));


    // To String impl
    let circle = Circle { radius: 14 };
    println!("{}", circle.to_string());

    // Parsing strings
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parse = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parse;
    println!("Sum is {}", sum);
}
