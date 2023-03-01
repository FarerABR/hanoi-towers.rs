fn main() {
    let mut from: Vec<u8> = vec![101, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut dest: Vec<u8> = Vec::new();
    dest.push(102);
    let mut by: Vec<u8> = Vec::new();
    by.push(103);
    solve(from.len() - 1, &mut from, &mut dest, &mut by);
}

fn solve(n: usize, from: &mut Vec<u8>, dest: &mut Vec<u8>, by: &mut Vec<u8>) {
    if n == 0 {
        return;
    }
    solve(n - 1, from, by, dest);

    dest.push(from.pop().unwrap());
    sort_poles(&from, &dest, &by);

    solve(n - 1, by, dest, from);
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

    // println!(
    //     "Source: {:?}\nTarget :{:?}\nBy: {:?}\n--",
    //     left, middle, right
    // );

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
}

fn sort_poles(in1: &Vec<u8>, in2: &Vec<u8>, in3: &Vec<u8>) {
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
