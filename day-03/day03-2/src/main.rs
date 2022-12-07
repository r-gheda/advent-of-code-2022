use std::io::prelude::*;
use itertools::Itertools;

fn convert(ch: char) -> i32
{
    let mut res = 0;
    if ch >= 'a' && ch <= 'z'
    { res = ch as i32 - ('a' as i32) + 1; }
    else
    { res = ch as i32 - ('A' as i32) + 27; }

    return res;
}

fn main() 
{
    let file = std::fs::File::open("input.txt")
        .expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    let mut res = 0;

    for (a, b, c) in reader.lines().tuples()
    {
        let a = a.unwrap();
        let b = b.unwrap();
        let c = c.unwrap();

        for ch in a.chars()
        {
            if b.contains(ch) && c.contains(ch)
            {
                res = res + convert(ch);
                break;
            }
        }
    }

    println!("{}", res);
}
