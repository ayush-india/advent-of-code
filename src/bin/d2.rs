use std::fs::*;
use std::io::*;
fn main() -> Result<()> {
    let mut f = File::open("input/d2")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    p1(content);
    Ok(())
}

fn p1(con: String) {
    let mut sum: Vec<i32> = Vec::new();
    let mut c = true;

    for line in con.lines() {
        let line_to = &line[7..];
        c = true;
        for (i, ch) in line_to.chars().enumerate() {
            // u can use match here
            match ch {
                'r' => {
                    let no = line_to[i - 3..i - 1].trim();
                    let pp = no.parse::<i32>().unwrap();
                    if pp > 12 {
                        println!("r {pp} ");
                        c = false;
                    }
                },
                'g' => {
                    let no = line_to[i - 3..i - 1].trim();
                    let pp = no.parse::<i32>().unwrap();
                    if pp > 13 {
                        println!("g {pp}");
                        c = false;
                    }
                },
                'b' => {
                    let no = line_to[i - 3..i - 1].trim();
                    let pp = no.parse::<i32>().unwrap();
                    if pp > 14 {
                        println!("b {pp}");
                        c = false;
                    }
                },
                _ => (),
            };
        }
        if c {
            sum.push(line[5..8].replace(":", "").trim().parse::<i32>().unwrap());
        }
    }
    println!("{:?}", sum.iter().sum::<i32>());
}
