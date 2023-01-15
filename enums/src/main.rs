enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value(&self) -> u8 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter => 25,
        }
    }
}

fn main() {
    let penny = Coin::Penny;
    let quarter = Coin::Quarter;
    println!("{}", value_in_cents(penny));
    println!("{}", quarter.value());

    main_2();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String)
}
fn main_2() {
    let x = Either::Right(String::from("Hello world"));
    let value = match x {
        Either::Left(n) => n,
        Either::Right(s) => s.len()
    };
    println!("{x:?} {value}");
}