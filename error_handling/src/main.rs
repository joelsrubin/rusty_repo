use std::io;
use std::io::Read;
use std::fs::File;


pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => {
            println!("string is {}", s);
            Ok(s)
        },
        Err(e) => Err(e)
    }
}
fn main() {


}