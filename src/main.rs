use std::fs;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("./planet.txt").unwrap();
    for line in contents.lines() {
        println!("This line is {line}");
    }
    let contents = fs::read("./planet.txt").unwrap();
    println!("The contents include {contents:?}");

    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planet.txt").unwrap();
    file.write(b"\nPluto");

}
