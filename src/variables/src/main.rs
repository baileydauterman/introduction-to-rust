fn main()
{
    // const is similar to defining a variable with the let keyword
    // but a const requires type definition for the var
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // let x = 5; - if you want to change a variable you must use the mut keyword
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");

    // Shadowing is the concept that the same variable name can be reassigned 
    // The compiler sees the most recent assigned variable during compilation
    

    let x = 68;
    let x = x + 1;

    {
        let x = x * 14;
        println!("The value of x in the inner scope is: {x}");

    }

    println!("The value of x is: {x}");

    // This effectively creates a new variable where different types can be 
    // assigned to it. This is done by using the let keyword to create the 
    // new var. 
    // This would not work:
    // let spaces = "     ";
    // spaces = spaces.len();

    let spaces = "     ";
    let spaces = spaces.len();

    // char values
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuples
    let tup = (69, 6.9, 420);
    let (x,y,z) = tup;
    
    // to access a tuple at a given index it will be tup.0
    println!("The values of x, y, z: {x}, {y}, {z}"); 

    // arrays
    // indexed like every other good programming language: months[0]
    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    
    // specifies the type in the array
    let thing: [i32; 5] = [1,2,3,4,5];

    // specifies the contents of the array
    // [3,3,3,3,3]
    let a = [3; 5];

    let ret_val = add_number(100, 69);
    println!("The added value is: {ret_val}");

    if ret_val > 200
    {
        println!("Nice your number is large than 200!");
    }
    else
    {
        println!("Bozo, your number isn't high enough");
    }

    let condition = true;

    let number = if condition {5} else {6};
}

fn write_values(x: i32)
{
    println!("The written value is: {x}");
}

fn add_number(x: i32, y: i32) -> i32
{
    return x + y;
}

fn loop_forever()
{
    loop {
        println!("Throw it back");
    }
}

fn loop_until_number_hit(break_num: i32) -> i32
{
    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;

        if counter == break_num {
            break counter * 2;
        }
    };

    println!("Reached the break num for the counter"); 

    return counter;
}

fn how_while_loops_work()
{
    let mut num = 0;

    while num != 10
    {
        println!("Current num {num}");
        num += 1;
    }

    println!("Reached the end of the loop");
}

fn while_loops_with_indexes()
{
    let a = [10, 20, 30, 40, 50, 60, 70, 80 , 90];
    let mut index = 0;

    while index != a.len()
    {
        println!("value of the array is: {}", a[index]);
        index += 1;
    }
}

fn foreach_loop()
{
    let a = [10, 20, 30, 40, 50, 60, 70, 80 , 90];

    for elem in a
    {
        println!("value is: {elem}");
    }
}

fn countdown(start: i32)
{
    for num in (1..start).rev()
    {
        println!("{num}");
    }

    println!("LIFTOFF!!");
}

fn string_builder(value: String)
{
    let mut s = String::from("hello");

    s.push_str(&value);

    println!("{s}");
}

// Instead of pass the string into this method we only pass a pointer
// therefore we don't hve to pass ownership into the function we just
// pass the memory location.
// Important to note that we cannot change the string from this scope,
// can only read what is at the memory location. 

// let s = String::from("this string"); 
// length_from_reference(&s);
fn length_from_reference(this_string: &String) -> usize
{
    return this_string.len();
}

// We cannot make more than one reference to a location. Must let one
// reference get out of scope
// If we want to change the value of the reference to this location we 
// must do this:
fn change_value_from_reference(this_string: &mut String)
{
    this_string.push_str(", added string stuff");
}

fn first_word_index(string: &str) -> usize
{
    let bytes = string.as_bytes();

    for (i, &character) in bytes.iter().enumerate()
    {
        if character == b' '
        {
            return i;
        }
    }

    return string.len();
}

// use &str as the param type because it allows use on String and str
fn first_word(string: &str) -> &str
{
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &string[..i];
        }
    }

    return &string[..]
}

fn slice_hello_world()
{
    let s = String::from("Hello, world!");

    let hello = &s[0..5];
    let world = &s[7..12];
}

fn compare_slices()
{
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}