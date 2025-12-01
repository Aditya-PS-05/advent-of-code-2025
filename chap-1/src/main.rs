// Read the input file

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>>{

    let input_path: &str = "input.txt";
    let f= File::open(input_path)?;

    let reader = BufReader::new(f);

    let mut dial= 50;
    let mut ans = 0;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let flag = line.chars().nth(0).unwrap();
        let num = line[1..].parse().unwrap_or(0);

        if flag == 'L' {
            dial -= num;
            
            dial = ((dial % 100) + 100) % 100;
        } else {
            dial += num;

            dial = dial % 100;
        }

        if dial == 0 {
            ans += 1;
        }
    }

    println!("The number is {:?}", ans);
    Ok(())
}
