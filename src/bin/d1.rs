use std::fs::*;
use std::io::*;
fn main() -> Result<()> {
    p1();
    Ok(())
}
fn p1() -> Result<()> {
    let mut f = File::open("input/d1")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let mut sum: u32 = 0;
    for line in content.lines() {
        let len = line.len();
        let mut c_isd = false;
        let mut char_isd = false;
        for (j, char) in line.chars().enumerate() {
            let b: u8 = line.as_bytes()[len - j - 1];
            let c: char = b as char;
            let c: char = line.as_bytes()[len - j - 1] as char;
            if char.is_digit(10) || c.is_digit(10) {
                if char.is_digit(10) && char_isd != true {
                    char_isd = true;
                    sum = sum + char.to_digit(10).unwrap() * 10;
                }
                if c.is_digit(10) && c_isd != true {
                    c_isd = true;
                    sum = sum + c.to_digit(10).unwrap();
                }
                if char_isd == true && c_isd == true {
                    break;
                }
            }
        }
    }
    println!("{sum}");
    Ok(())
}
