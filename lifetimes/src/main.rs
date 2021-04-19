use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
let string1 = String::from("abcd");
let string2 = "xyz";

let result = longest_with_an_announcement(string1.as_str(), string2, "Hello World");
println!("The longest string is {}", result);

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.')
    .next()
    .expect("Could not find a '.'");
let i = ImportantExcerpt {part: first_sentence};
println!("{:#?}", i);

    let r;
    let x = 5;
    r = &x;


    println!("r: {}", r)
}
