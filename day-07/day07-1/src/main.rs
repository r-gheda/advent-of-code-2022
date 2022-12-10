use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let file = std::fs::File::open("input.txt")
        .expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    
    let mut folder_size = HashMap::<String, i32>::new();
    let mut folder_path = Vec::<String>::new();

    for (_idx, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        let mut command = line.split(" ");
        
        let first = command.nth(0);
        let second = command.nth(0);
        let third = command.nth(0);
        
        match first
        {
            Some(x) => {               
                if x.eq("$")
                {
                    match second
                    {
                        Some(y) => {
                            match third
                            {
                                Some(z) => {
                                    if y.eq("cd")
                                    {
                                        if z.eq("..") { folder_path.pop(); }
                                        else if z.eq("/") { folder_path = Vec::new(); folder_path.push(String::from("/")); }
                                        else { folder_path.push(String::from(z)); }
                                    } 
                                    else { panic!("Unrecognized argument {}", y); }
                                },

                                None => continue,
                            }
                        },

                        None => panic!("Invalid string y")
                    }
                    
                } else if x.ne("dir")
                {
                    let mut curr_path = String::new();

                    for folder in &folder_path
                    {
                        if (*curr_path).ne("/") && (*folder).ne("/") { curr_path = String::from(curr_path) + "/"; }
                        curr_path = curr_path.to_owned() + folder;
                        let file_size = x.parse::<i32>().unwrap();
                        let tmp = curr_path.to_owned();
                        match folder_size.get(&tmp)
                        {
                            Some(size) => folder_size.insert(tmp, *size + file_size),
                            None => folder_size.insert(tmp, file_size),
                        };
                    }
                }
            },

            None => panic!("Invalid string x")
        }

    }
    let mut res = 0;

    for (_key, size) in folder_size
    {
        if size < 100000 {res = res + size};
    }
    println!("Solution: {}", res);
}
