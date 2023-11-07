use std::io;
use std::fs::File;
use std::io::Read;

fn read_username_from_flie() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match read_username_from_flie() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
}