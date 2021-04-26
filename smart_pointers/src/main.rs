
/*
smart pointers
originate in C++
- have unique metadata
- one we will explore is the reference counting smart pointer type
- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners
- Box<T> allows immutable or mutable borrows checked at compile time
- Rc<T> allows only immutable borrows checked at compile time
- RefCell<T> allows immutable or mutable borrows checked at runtime
- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate hte value inside the RefCell<T> even when the RefCell<T> is immutable
- this is known as the _interior mutability_ pattern

 */

 use::std::ops::Deref;

 #[derive(Debug)]
struct MyBox<T>(T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }

}

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}


enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;


fn main() {

    let mut x = 5;
    let y = &mut x;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let c = CustomSmartPointer {data: String::from("my stuff") };
    let d = CustomSmartPointer {data: String::from("other stuff") };
    println!("CustomSmartPointers created.");


 let m = MyBox::new(String::from("Rust"));
 println!("{:?}", hello(&m));
 let x = 5;
 let y = MyBox(x);

 assert_eq!(5, *y);
}

