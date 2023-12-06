use std::fs::*;
use std::io::*;
fn main() -> Result<()> {
    // p1();
    p2();
    Ok(())
}

fn p2() -> Result<()> {
    let mut f = File::open("input/d1")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let mut da_textier: Vec<char> = content.chars().collect();
    let num_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for i in 0..10 {
        // makes the 2nd letter of each digit word into the digit itself
        let mut loc = content.find(num_words[i]);
        while loc.is_some() {
            da_textier[loc.unwrap() + 1] = char::from_digit(i as u32, 10).unwrap();
            content = da_textier.clone().into_iter().collect();
            loc = content.find(num_words[i])
        }
    }
    let full: Vec<&str> = content.split("\n").collect();
    //split string into chars, just take the chars that are digits, multiply first by 10, add to last
    let mut total: u32 = 0;
    for input in full.iter() {
        let nums: Vec<u32> = input
            .chars()
            .filter(|&x| '0' <= x && '9' >= x)
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        println!("{:?}", nums);
        if nums == [] {
            continue;
        }
        total += nums[0] * 10 + nums[nums.len() - 1];
    }
    println!("{total}");
    Ok(())
}

fn p1() -> Result<()> {
    let mut f = File::open("input/d1")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let mut sum: u32 = 0;
    for line in content.lines() {
        let mut nos = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                nos.push(char);
            }
        }

        if nos.len() == 1 {
            sum += nos[0].to_digit(10).unwrap() * 10 + nos[0].to_digit(10).unwrap();
        } else if nos.len() == 0 {
            continue;
        } else {
            sum += nos[0].to_digit(10).unwrap() * 10 + nos[nos.len() - 1].to_digit(10).unwrap();
        }
    }
    println!("{sum}");
    Ok(())
}
