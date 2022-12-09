use std::fs;
use std::collections::HashSet;

fn main()
{
    let file = fs::read_to_string("input.txt")
        .expect("Failed to open file");

    let mut counter = 0;
    let packet_dim = 14;

    for _ in file.chars()
    {
        let mut chars: HashSet<char> = HashSet::new();
        let mut is_packet = true;

        for i in 0..packet_dim
        {
            if i > counter { is_packet = false; break; }
            let ch = file.chars().nth(counter-i);
            match ch
            {
                Some(ch) => {
                    if !chars.contains(&ch) { chars.insert(ch); } 
                    else { is_packet = false; break; }
                },

                None => panic!("No charachter read")
            }
        }
        counter = counter + 1;
        if is_packet { break; }
    }

    println!("Solution: {}", counter);
}
