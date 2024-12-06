use std::fs;

fn split_strings_from_file() -> Vec<Vec<i32>> {
    let contents =
        fs::read_to_string("src/inputs/day2.txt").expect("Something went wrong reading the file");

    let split: Vec<&str> = contents.lines().collect();

    let mut list: Vec<Vec<i32>> = Vec::new();

    split.iter().for_each(|i| {
        let mut temp: Vec<i32> = Vec::new();
        i.split_whitespace().for_each(|j| {
            temp.push(str::parse::<i32>(j).unwrap());
        });
        list.push(temp);
    });

    list
}

fn is_conformant(list: &[i32]) -> bool {
    let min = 1;
    let max = 3;
    let is_increasing: bool = list[0] < list[1];
    let mut valid = false;

    for (i, item) in list.iter().enumerate() {
        if i == list.len() - 1 {
            break;
        };
        let diff = list[i + 1] - item;
        if is_increasing {
            valid = diff > 0 && diff >= min && diff <= max;
        } else {
            valid = diff < 0 && diff.abs() >= min && diff.abs() <= max;
        }
        if !valid {
            break;
        }
    }
    valid
}

pub fn solve_part_1() {
    let list = split_strings_from_file();

    let mut total = 0;

    // reports is a list of ints
    for reports in list.iter() {
        let valid = is_conformant(reports);

        if valid {
            total += 1
        }
    }
    println!("day 2 - part 1 total: {}", total);
}

pub fn solve_part_2() {
    let list = split_strings_from_file();

    let mut total = 0;

    // reports is a list of ints
    for reports in list.iter() {
        for i in 0..reports.len() {
            let mut new_list = reports.to_vec();
            new_list.remove(i);
            let valid = is_conformant(&new_list);
            if valid {
                total += 1;
                break;
            }
        }
    }
    println!("day 2 - part 2 total: {}", total);
}
