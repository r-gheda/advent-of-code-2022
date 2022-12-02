use std::io::prelude::*;

fn rock(opp: char) -> i32
{
    let mut points = 1;

    if opp == 'C' {points += 6;}
    else if opp == 'A' {points += 3;}

    return points;
}

fn paper(opp: char) -> i32
{
    let mut points = 2;

    if opp == 'A' {points += 6;}
    else if opp == 'B' {points += 3;}

    points
}

fn scissors(opp: char) -> i32
{
    let mut points = 3;

    if opp == 'B' {points += 6;}
    else if opp == 'C' {points += 3;}

    points
}

fn main() {
    let file = std::fs::File::open("input.txt")
        .expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    let mut points: i32 = 0;
    
    for (_idx, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        match line.chars().nth(2)
        {
            Some(x) => {
                match line.chars().nth(0)
                {
                    Some(y) => {
                        match x 
                        {
                            'X' => points += rock(y),
                            'Y' => points += paper(y),
                            'Z' => points += scissors(y),

                            _ => panic!("Input not recognized!")
                        }
                    }

                    None => panic!("Input not recognized!")
                }
                
            }

            None => panic!("Input not recognized"),
        }
    }

    println!("Solution: {}", points);
}
