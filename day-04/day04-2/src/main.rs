use std::io::prelude::*;

fn overlaps(range_1: &str, range_2: &str) -> bool
{
    let range_1: Vec<&str> = range_1.split("-").collect();
    let range_2: Vec<&str> = range_2.split("-").collect();
    let start_1 = range_1[0].parse::<i32>().unwrap();
    let end_1 = range_1[1].parse::<i32>().unwrap();
    let start_2 = range_2[0].parse::<i32>().unwrap();
    let end_2 = range_2[1].parse::<i32>().unwrap();

    return ((start_1 <= end_2) && (end_1 >= start_2)) || ((start_2 <= end_1) && (end_2 >= start_1));
}   

fn main() 
{
    let file = std::fs::File::open("input.txt")
        .expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    let mut counter = 0;

    for (_idx, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        let ranges: Vec<&str> = line.split(",").collect();
        let range_1 = ranges[0];
        let range_2 = ranges[1];

        if overlaps(range_1, range_2) { counter = counter + 1; }
    }

    println!("{}", counter);
}