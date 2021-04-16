
fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    for i in v {
        v1.push(i);
    }
    println!("the vector looks like this: {:?}", v1)

}
