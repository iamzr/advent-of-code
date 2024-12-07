use std::fs;

fn main() {
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    let values = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| s.split(":").collect::<Vec<&str>>())
        .map(|row| {
            println!("{:?}", row);
            (
                row[0].parse::<i64>().expect("Invalid Number"),
                row[1]
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().expect("Invalid Number"))
                    .collect::<Vec<i64>>(),
            )
        })
        .collect::<Vec<(i64, Vec<i64>)>>();

    println!("{:?}", values);

    let mut ans = 0;


    for (target, l) in values {
        // let l_rv = l.iter().copied().rev().collect::<Vec<i64>>();

        let v = find(target, 0, 0, &l);
        println!("For target {} we found {}", target, v);

        if v {
            ans += target;
        }
    }

    println!("Final answer: {}", ans);
}

/// For a given target there's two things you can do
fn find(target: i64, curr: i64, idx: usize, l: &Vec<i64>) -> bool {
    if curr > target {
        // println!("greater than target {}, curr {}", target, curr);
        return false
    }

    if curr == target {
        return true
    }


    if idx >= l.len() {
        // println!(" Got to end for target {} with curr {}", target, curr);
        return false
    }

    let n= l[idx];
    // println!("{}", n);

    let check_1 = find(target, curr + n, idx +1, l);
    // println!("Additive path {}", check_1);
    let check_2 = find(target, curr * n, idx + 1, l);
    // println!("Multiplicative path {}", check_2);

    check_1 || check_2

}
