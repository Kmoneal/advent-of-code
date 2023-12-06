use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut total:i64;
    let mut grand_total: i64 = 0;


    for line in reader.lines() {
        let my_chars: Vec<_> = line.unwrap().chars().collect();
        let mut first:i64 = -1;
        let mut last:i64 = -1;
        for c in my_chars {
            if c.is_digit(10) && first == -1 {
                first = i64::from(c.to_digit(10).unwrap());
            }
            if c.is_digit(10) {
                last = i64::from(c.to_digit(10).unwrap());
            }
        }
        
        total = first * 10 + last;
        grand_total = grand_total + total;
    }
    println!("Part 1: {}", grand_total);

    Ok(())
}