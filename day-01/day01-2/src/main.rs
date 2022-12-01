use std::io::prelude::*;
use priority_queue::PriorityQueue;

fn main()
{ 
    let file = std::fs::File::open("input.txt")
        .expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    let mut pq = PriorityQueue::new();
    let mut current_elf: i32 = 0;

    for (idx, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        if line.eq("\n") || line.eq("")
        {
            pq.push(idx, current_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line.parse::<i32>().unwrap();
    }

    let mut total: i32 = 0;
    println!("Solution: ");
    for _ in 0..3
    {
        let val = pq.pop();
        match val 
        {
            Some(x) => {
                println!("Elf: {} Calories: {}", x.0, x.1);
                total += x.1
            }
            None => println!("Empty queue")
        }
    }

    println!("Total: {}", total);
}