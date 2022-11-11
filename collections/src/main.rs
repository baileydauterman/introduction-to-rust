use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    for i in 5..=10
    {
        v.push(i);
    }

    for vector in &v
    {
        println!("{vector}");
    }

    // can edit the vector in place
    for ve in &mut v
    {
        *ve += 3;
    }

    let v = v;

    let third: &i32 = &v[2];

    let third: Option<&i32> = v.get(2);

    print_value(third);

    let does_not_exist: Option<&i32> = v.get(100);

    print_value(does_not_exist);

    // about strings
    let s = "initial contents".to_string();

    let mut data = String::from("bar");
    data.push_str("baz");

    create_hashmap();
}


fn print_value(index: Option<&i32>)
{
    match index
    {
        Some(index) => println!("The elem is: {index}"),
        None => println!("That elem doesn't exist")
    }
}

fn create_hashmap()
{
   let mut scores = HashMap::new();

   scores.insert(String::from("Blue"), 10);
   scores.insert(String::from("Yellow"), 50);

   let team_name = String::from("Blue");
   let score = scores.get(&team_name).copied().unwrap_or(0);

   println!("{score}");
}
