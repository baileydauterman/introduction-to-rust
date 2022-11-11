enum IpAddressKind
{
    IPv4, 
    IPv6,
}

enum IpAddr {
    IPv4(String),
    IPv6(String),
}

struct IpAddress
{
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let four = IpAddressKind::IPv4;
    let six = IpAddressKind::IPv6;


    let home = IpAddress {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: six,
        address: String::from("::1"),
    };

    let home = IpAddr::IPv4(String::from("127.0.0.1"));
    let loopback = IpAddr::IPv6(String::from("::1"));
}


enum Coin {
    Penny, Nickel, Dime, Quarter, HalfDollar
}

fn value_of_coin(coin: Coin) -> usize
{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::HalfDollar => 50
    }
}
