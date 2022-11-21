fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    return largest;
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest<T>(list: &[T]) -> &T
{
    let mut largest = &list[0];

    for item in list
    {
        if item > largest
        {
            largest = item;
        }
    }

    return largest;
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest i32 was: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char was: {result}");

    let result = largest(&char_list);
    println!("The largest char is: {result}");

    let result = largest(&number_list);
    println!("The largest i32 is: {result}");

} 
