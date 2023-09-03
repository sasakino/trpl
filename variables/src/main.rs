use std::io;

fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';    //ハート目の猫
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred: {}, six_point_four: {}, one: {}",
        five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("first: {}, second: {}", first, second);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index enterd was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
