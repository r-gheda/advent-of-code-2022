use std::io::prelude::*;

fn lose(opp: char) -> i32
{
    let mut points = 0;

    match opp
    {
        'A' => points += 3,
        'B' => points += 1,
        'C' => points += 2,

        _ => panic!("Input not recognized!")
    }

    return points;
}

fn draw(opp: char) -> i32
{
    let mut points = 3;

    match opp
    {
        'A' => points += 1,
        'B' => points += 2,
        'C' => points += 3,

        _ => panic!("Input not recognized!")
    }

    points
}

fn win(opp: char) -> i32
{
    let mut points = 6;

    match opp
    {
        'A' => points += 2,
        'B' => points += 3,
        'C' => points += 1,

        _ => panic!("Input not recognized!")
    }

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
                            'X' => points += lose(y),
                            'Y' => points += draw(y),
                            'Z' => points += win(y),

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
