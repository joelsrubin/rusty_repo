#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }

    fn double(&self) -> Rectangle {
        Rectangle {
            width: self.width * 2,
            height: self.height * 2
        }
    }
}

#[derive(Debug)]
enum UsState {
    AK, AL, AR, AS, AZ, CA, CO, CT, DC, DE, FL, GA, GU, HI, IA, ID, IL, IN, KS, KY, LA, MA, MD, ME, MI, MN, MO, MP, MS, MT, NC, ND, NE, NH, NJ, NM, NV, NY, OH, OK, OR, PA, PR, RI, SC, SD, TN, TX, UM, UT, VA, VI, VT, WA, WI, WV, WY
}
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },

    }
}

fn count_nonquarter_coins(coin: Coin) -> u32 {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        return count;
    } else {
        count += 1;
       return count;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {

    // structs

    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    let sq = Rectangle::square(30);
    let doubled = Rectangle::double(&rect2);

    println!("Your first square: {:#?}", sq);
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.", rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can doubled {:?} hold sq {:?} ? {}", doubled, sq, doubled.can_hold(&sq));
    println!("{:#?} doubled is {:#?}", rect1, doubled);
    println!("{:#?} doubled is {:#?}", rect1, rect1.double());


    // enums

    let five = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
    println!("5 plus 1 = {:#?}", plus_one(five));
    // let sum = x + y; can't add an option to an i8 here. need to use match so pattern match and derive the T value of Option<T>
    let value = value_in_cents(Coin::Penny);
    println!("The value is {}", value);
    println!("The value of a nickle is {}", value_in_cents(Coin::Nickle));
    println!("{}", value_in_cents(Coin::Quarter(UsState::AK)));
    println!("The current count is {}", count_nonquarter_coins(Coin::Penny));
    println!("The current count is {}", count_nonquarter_coins(Coin::Penny));
    println!("The current count is {}", count_nonquarter_coins(Coin::Penny))
}



