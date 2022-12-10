use std::io::prelude::*;

fn is_visible(trees: Vec<Vec<u32>>, row: usize, col: usize) -> bool
{
    if row == 0 || col == 0 || row == trees.len()-1 || col == trees[row].len()-1
    { return true; }

    let height = trees[row][col];

    for i in 0..row
    {
        if trees[i][col] >= height{ break; }
        if i == row-1 { return true; }
    }

    for i in row+1..trees.len()
    {
        if trees[i][col] >= height{ break; }
        if i == trees.len()-1 { return true; }
    }

    for j in 0..col
    {
        if trees[row][j] >= height{ break; }
        if j == col-1 { return true; }
    }

    for j in col+1..trees.len()
    {
        if trees[row][j] >= height{ break; }
        if j == trees[row].len()-1 { return true; }
    }

    return false;
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

    let mut counter = 0;
    let rows = trees.len();
    for row in 0..rows
    {
        let cols = trees[row].len();
        for col in 0..cols
        {
            let res: bool;
            res = is_visible(trees.clone(), row, col);
            visibles[row][col] = res;
            if res { counter = counter + 1; }
        }
    }

    println!("Solution: {}", counter);
}
