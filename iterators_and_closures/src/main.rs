/*
    Notes:
    - Closures: a function-like construct you can store in a variable
    - Iterators: a way of processing a series of elements
    ~ Rust's closures are anon funcs. Unlike functions, they can capture values from the scope in which they're called
    ~ Let's make a function that can simulate generating a custom exercise plan

    ~ To make a struct that holds a closure, we need to specify the type of the closure
    ~ Each closure instance has its own unique anonymous type: that is even if two clsoures have the same signature, their types are still considered different
    ~ To define structs, enums, or function parameters that use closures, we use generics and trait bounds
    ~ The Fn traits are provided by the std library

    ~ All closure implement at least one of the following: Fn, FnMut, or FnOnce
    ~ Here we use Fn - to represent the types of the parameters and return values the closures must have to match this trait bound eg Fn(u32) -> u32
*/

use std::{thread, u32};
use std::time::Duration;


// a function that takes two seconds to 'calculate' and then returns passed in num
fn _simulated_expensive_calculation(intensity: u32) -> u32 {
println!("calculating slowly...");
thread::sleep(Duration::from_secs(2));
intensity
}
#[derive(Debug)]// example of a Cacher struct
struct Cacher <T>
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // this method checks whether we already have a resulting value in self.value in a Some
        // if we do, it returns teh value within the Some w/o executing closure again
        // if self.value is None, code calls the closure stored in self.calculation,
            // saves it in self.value and returns the value as well
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}



fn generate_workout(intensity: u32, random_number: u32) {
    // a closure with the same functionality as above ^
let mut expensive_result = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
});


    if intensity < 25 {
        println!("Today, do {} pushups",
        expensive_result.value(intensity)
    );
        println!("Next, do {} situps",
        expensive_result.value(intensity)
    );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}





fn main() {

    let v1 : Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v1: {:?} mapped is {:?}", v1, v2);



    // required inputs: an intensity number from the user, a random number
    let simulated_user_specified_value = 20;
    let simluated_random_number = 5;

    generate_workout(
        simulated_user_specified_value,
        simluated_random_number
    );


    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("{}", equal_to_x(y));


    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;


    let y = vec![1, 2, 3];
    println!("{}", equal_to_x(y));


    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val)
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a|a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    println!("{:#?}", v1);
    assert_eq!(v2, 1)
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker") },
        Shoe {size: 13, style: String::from("sandal") },
        Shoe {size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size, vec![
            Shoe {size: 10, style: String::from("sneaker") },
            Shoe {size: 10, style: String::from("boot") },
        ]
    )
}
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    println!("{:#?}", counter);
    assert_eq!(counter.next(), Some(1));
    println!("{:#?}", counter);
    assert_eq!(counter.next(), Some(2));
    println!("{:#?}", counter);
    assert_eq!(counter.next(), Some(3));
    println!("{:#?}", counter);
    assert_eq!(counter.next(), Some(4));
    println!("{:#?}", counter);
    assert_eq!(counter.next(), Some(5));
    println!("{:#?}", counter);
    assert_eq!(counter.next(), None);
    println!("{:#?}", counter)
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                    .map(|(a,b)| a * b)
                                    .filter(|x| x % 3 == 0)
                                    .sum();
    assert_eq!(18, sum);
}