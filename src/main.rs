use std::env;
use std::num::{ParseFloatError, ParseIntError};
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum ErrType {
    NotEnoughArguments(String),
    NegativeErr(String),
    SpeedErr(String),
    ParseIErr(ParseIntError),
    ParseFErr(ParseFloatError),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(x) = config(&args) {
        println!("Exited with Error: {:?}", x);
        return;
    }
    let (n, sp) = config(&args).unwrap();

    let mut from: Vec<u8> = Vec::new();
    from.push(101);
    (0..n).for_each(|i| from.push((n - i) as u8));
    let mut dest: Vec<u8> = Vec::new();
    dest.push(102);
    let mut by: Vec<u8> = Vec::new();
    by.push(103);
    solve(n, &mut from, &mut dest, &mut by, sp);
}

fn solve(n: usize, from: &mut Vec<u8>, dest: &mut Vec<u8>, by: &mut Vec<u8>, sp: f32) {
    if n == 0 {
        return;
    }
    solve(n - 1, from, by, dest, sp);

    dest.push(from.pop().unwrap());
    if sp == 0.0 && n == 1 {
        sort_display(&from, &dest, &by);
    } else {
        sort_display(&from, &dest, &by);
        std::thread::sleep(Duration::from_secs_f32(sp));
    }

    solve(n - 1, by, dest, from, sp);
}

fn display(l_pole: &Vec<u8>, m_pole: &Vec<u8>, r_pole: &Vec<u8>) {
    print!("\x1B[2J");

    let mut left = l_pole.clone();
    left.rotate_left(1);
    left.pop();
    let mut middle = m_pole.clone();
    middle.rotate_left(1);
    middle.pop();
    let mut right = r_pole.clone();
    right.rotate_left(1);
    right.pop();

    // left , middle , right
    /*
    |            |            |
    |            |            |
    |            |            |
    |            |            |
    5-----       |            |
    6------      |            |
    7-------     3---         1-
    8--------    4----        2--
    */

    let len = left.len() + middle.len() + right.len();

    (0..len).for_each(|i| {
        print!("        ");
        let index = len - i - 1;

        if left.len() <= index {
            print!("|");
            (0..len as u8 + 4).for_each(|_| {
                print!(" ");
            });
            if len > 9 {
                print!(" ");
            }
        } else {
            print!("{}", left[index]);
            (0..left[index]).for_each(|_| {
                print!("-");
            });
            (0..(len as u8 - left[index] + 4)).for_each(|_| {
                print!(" ");
            });
            if left[index] < 10 && len > 9 {
                print!(" ");
            }
        }

        if middle.len() <= index {
            print!("|");
            (0..len as u8 + 4).for_each(|_| {
                print!(" ");
            });
            if len > 9 {
                print!(" ");
            }
        } else {
            print!("{}", middle[index]);
            (0..middle[index]).for_each(|_| {
                print!("-");
            });
            (0..(len as u8 - middle[index] + 4)).for_each(|_| {
                print!(" ");
            });
            if middle[index] < 10 && len > 9 {
                print!(" ");
            }
        }

        if right.len() <= index {
            print!("|");
        } else {
            print!("{}", right[index]);
            (0..right[index]).for_each(|_| {
                print!("-");
            });
        }
        print!("\n");
    });
    // println!("       Left            Middle           Right");
}

fn sort_display(in1: &Vec<u8>, in2: &Vec<u8>, in3: &Vec<u8>) {
    if in1[0] == 101 {
        if in2[0] == 102 {
            display(in1, in2, in3);
        } else {
            display(in1, in3, in2);
        }
    } else if in2[0] == 101 {
        if in1[0] == 102 {
            display(in2, in1, in3);
        } else {
            display(in2, in3, in1);
        }
    } else {
        if in1[0] == 102 {
            display(in3, in1, in2);
        } else {
            display(in3, in2, in1);
        }
    }
}

fn config(args: &Vec<String>) -> Result<(usize, f32), ErrType> {
    if args.len() < 3 {
        return Err(ErrType::NotEnoughArguments("Not enough arguments".into()));
    }
    if let Err(x) = args[1].parse::<usize>() {
        return Err(ErrType::ParseIErr(x));
    }
    if let Err(x) = args[2].parse::<f32>() {
        return Err(ErrType::ParseFErr(x));
    }
    let n = args[1].parse::<usize>().unwrap();
    let sp = args[2].parse::<f32>().unwrap();
    if n < 1 {
        return Err(ErrType::NegativeErr(
            "Number of disk can not be negative".into(),
        ));
    }
    if sp < 0.0 || sp > 1.0 {
        return Err(ErrType::SpeedErr(
            "Speed must be between 0 and 10(sec), choose 0 to see the full speed".into(),
        ));
    }

    return Ok((n, sp));
}
