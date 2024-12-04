use std::fs;

fn main() {
    let file_path = "./input.txt"; 

    let contents = fs::read_to_string(file_path)
        .unwrap();
    
    let values = contents.split("\n").collect::<Vec<&str>>();
    println!("{:?}", values);

    let reports : Vec<Vec<i32>> = values
        .iter()
        .map(|row| row.split_whitespace()
                        .map(|num| num.parse::<i32>().expect("Invalid Number"))
                        .collect::<Vec<i32>>()
                    )
                        .collect::<Vec<Vec<i32>>>();

    println!("{:?}", reports);

    let mut ans = vec!(0; reports.len());

    for (index, levels) in reports.into_iter().enumerate() {
        let mut prev = levels[0];

        let mut completed = true;
        
        let mut total_sign: i32 = 0;
        for curr in &levels[1..] {
            print!("{}, {}\n", prev, curr);
            let diff = curr - prev;
            let sign = match diff {
                        n if n > 0 => 1,
                        n if n < 0 => -1,
                        _ => 0,
                    };

            if sign == 0 {
                completed = false;
                println!("broke because not-increasing or decreasing");
                break;
            }

            total_sign += sign;
            
        
            if diff.abs() > 3 {
                completed = false;
                println!("broke because diff greater than 3");
                break;
            }
            prev = *curr;
        }

        println!("{:?} {:?}", total_sign, levels.len());
        if total_sign.abs() != levels.len() as i32 -1 {
            println!("broke because sign");
            continue; 
        }
        if completed {
            ans[index] = 1;
        }
    }

    println!("{:?}", ans);

    let result: i32 = ans.iter().sum();

    println!("{:?}", result);

}

// mod tests {
//     super:: *;

//     fn test_map_to_matrix {

//     }
// }
