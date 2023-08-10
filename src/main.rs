use std::io;
use std::io::Read;
use regex::Regex;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("beeg error"); 
    for line in buf.lines() {
        let re = Regex::new(r"[rl]").unwrap();
        let re2 = Regex::new(r"(oo)").unwrap();
        let result = re.replace_all(&line, "w");
        println!("{}", re2.replace_all(&result, "OwO"));
    }

}