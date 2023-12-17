use std::io::BufRead;

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut line = String::new();
    let mut sum: i32 = 0;
    let mut counter = 0;
    let pairs = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ];
    while let Ok(_) = stdin.lock().read_line(&mut line) {
        counter += 1;
        if line == "" {break;}
        let mut first_digit = '\0';
        let mut last_digit = '\0';
        

        for (index, ch) in line.char_indices() {
            for (word, num) in pairs {
                if word.starts_with(ch) && line.len() >= index+word.len() && &line[index..index+word.len()] == word{
                    if first_digit == '\0' {
                        first_digit = num;
                    }
                    last_digit = num;
                }
            }
            if !ch.is_ascii_digit() { continue; }
            if first_digit == '\0' {
                first_digit = ch;
            }
            last_digit = ch;
        }
        let num = String::from(format!("{}{}", first_digit, last_digit)).parse::<i32>()?;
        sum += num;
        println!("{}. {} - {} - {}", counter, line.trim(), num, sum);
        line.clear();
    }
    println!("{}", sum);
    Ok(())
}
