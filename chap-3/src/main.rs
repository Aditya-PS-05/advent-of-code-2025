use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn best_k_subsequence_as_number(digits: &[u32], k: usize) -> Option<u128> {
    let n = digits.len();
    if k == 0 || n < k {
        return None;
    }

    let mut start = 0usize;
    let mut remaining = k;
    let mut value: u128 = 0;

    while remaining > 0 {
        // We must pick an index in [start ..= n - remaining]
        let end_inclusive = n - remaining;
        // Find index of maximum digit in digits[start..=end_inclusive]
        let mut max_digit = 0u32;
        let mut max_idx = start;
        for i in start..=end_inclusive {
            let d = digits[i];
            if d > max_digit {
                max_digit = d;
                max_idx = i;
                // Early exit: if digit is 9 it's the best possible
                if max_digit == 9 { break; }
            }
        }

        value = value * 10 + (max_digit as u128);
        start = max_idx + 1;
        remaining -= 1;
    }

    Some(value)
}

fn main() -> Result<(), Box<dyn Error>> {
    const K: usize = 12;
    let reader = BufReader::new(File::open("input.txt")?);
    let mut total: u128 = 0;

    for line in reader.lines() {
        let line = line?;
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        if let Some(num) = best_k_subsequence_as_number(&digits, K) {
            total += num;
        } else {
            // If a line has fewer than K digits, we skip it.
            // Alternatively you could handle it as an error.
        }
    }

    println!("Total joltage (sum of best {}-digit numbers) = {}", K, total);
    Ok(())
}

