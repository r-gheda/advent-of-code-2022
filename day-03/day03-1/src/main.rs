use std::io::prelude::*;

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

    for (_idx, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        let length = line.chars().count();

        let (c1, c2) = line.split_at(length / 2);

        for ch in c1.chars()
        {
            if c2.contains(ch)
            {
                res = res + convert(ch);
                break;
            }
        }
    }

    println!("{}", res);
}
