use std::io::prelude::*;
use std::cmp::max;

fn get_score(trees: Vec<Vec<u32>>, row: usize, col: usize) -> i32
{
    let height = trees[row][col];
    let mut res = 1;
    
    for i in row+1..trees.len()
    {
        if trees[i][col] >= height { res = i-row; break;}
        if i == trees.len()-1 { res *= i-row; }
    }
    for i in 1..row+1
    {
        if trees[row-i][col] >= height { res *= i; break;}
        if i == row { res *= i; }
    }
    
    for i in col+1..trees[row].len()
    {
        if trees[row][i] >= height { res *= i-col; break;}
        if i == trees[row].len()-1 { res *= i-col; }
    }

    for i in 1..col+1
    {
        if trees[row][col-i] >= height { res *= i; break;}
        if i == col { res *= i; }
    }


    return res as i32;
}

fn main() {
    let file = std::fs::File::open("input.txt")
        .expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    
    let mut trees: Vec<Vec<u32>> = Vec::new();
    let mut visibles: Vec<Vec<bool>> = Vec::new();
    let mut iter= 0;

    for (_idx, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        trees.push(Vec::new());
        visibles.push(Vec::new());

        for i in 0..line.chars().count()
        {
            match line.chars().nth(i)
            {
                Some(x) =>{
                    trees[iter].push(x.to_digit(10).unwrap());
                    visibles[iter].push(false);
                },
                None => panic!("Failed to load character"),
            }
        }
        iter += 1;
    }

    let mut res = 0;

    for row in 0..trees.len()
    {
        for col in 0..trees[row].len()
        {
            let r = get_score(trees.clone(), row, col);
            res = max(res, r);
        }
    }

    println!("Solution: {}", res);
}
