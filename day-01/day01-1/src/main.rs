use std::io::prelude::*;

fn main()
{ 
    let file = std::fs::File::open("input.txt")
        .expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    let mut max_elf: i32 = 0;
    let mut current_elf: i32 = 0;

    for (_idx, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        if line.eq("\n") || line.eq("")
        {
            max_elf = std::cmp::max(current_elf, max_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line.parse::<i32>().unwrap();
    }

    println!("Solution: {}", max_elf)
}