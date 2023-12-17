use std::io::BufRead;

use anyhow::Result;

fn main() -> Result<()>{
    let stdin = std::io::stdin();
    let mut line = String::new();
    let mut sum: i32 = 0;
    let mut counter = 0;
    while let Ok(_) = stdin.lock().read_line(&mut line) {
        counter += 1;
        if line == "" {break;}
        let mut first_digit = '\0';
        let mut last_digit = '\0';
        
        for char in line.chars() {
            if !char.is_ascii_digit() { continue; }
            if first_digit == '\0' {
                first_digit = char;
            }
            last_digit = char;
        }
        let num = String::from(format!("{}{}", first_digit, last_digit)).parse::<i32>()?;
        sum += num;
        println!("{}. {} - {} - {}", counter, line.trim(), num, sum);
        line.clear();
    }
    println!("{}", sum);
    Ok(())
}
