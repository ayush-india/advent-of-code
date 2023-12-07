use std::fs::*;
use std::io::*;
fn main() -> Result<()> {
    let mut f = File::open("input/d2")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    // p1(content);
    p2(content); // the value of red is incrising by 1 for some reason see it;
    Ok(())
}

fn p2(con: String) {
    let mut sum: Vec<i32> = Vec::new();
    let mut r_l = 0;
    let mut g_l = 0;
    let mut b_l = 0;

    for line in con.lines() {
        let line_to = &line[7..];
        println!("{line_to}");
        r_l = 0;
        b_l = 0;
        g_l = 0;

        for (i, ch) in line_to.chars().enumerate() {
            // u can use match here
            match ch {
                'r' => {
                    let no = line_to[i - 3..i - 1].trim();
                    let pp = no.parse::<i32>().unwrap();
                    if pp >= r_l {
                        r_l = pp;
                    }
                }
                'g' => {
                    let no = line_to[i - 3..i - 1].trim();
                    let pp = no.parse::<i32>().unwrap();
                    if pp >= g_l {
                        g_l = pp;
                    }
                }
                'b' => {
                    let no = line_to[i - 3..i - 1].trim();
                    let pp = no.parse::<i32>().unwrap();
                    if pp >= b_l {
                        b_l = pp;
                    }
                }
                _ => (),
            };
        }
        println!("r {r_l}");
        println!("b {b_l}");
        println!("g {g_l}");
        sum.push(b_l*r_l*g_l);
    }
    println!("{:?}", sum.iter().sum::<i32>());
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
                }
                'g' => {
                    let no = line_to[i - 3..i - 1].trim();
                    let pp = no.parse::<i32>().unwrap();
                    if pp > 13 {
                        println!("g {pp}");
                        c = false;
                    }
                }
                'b' => {
                    let no = line_to[i - 3..i - 1].trim();
                    let pp = no.parse::<i32>().unwrap();
                    if pp > 14 {
                        println!("b {pp}");
                        c = false;
                    }
                }
                _ => (),
            };
        }
        if c {
            sum.push(line[5..8].replace(":", "").trim().parse::<i32>().unwrap());
        }
    }
    println!("{:?}", sum.iter().sum::<i32>());
}
