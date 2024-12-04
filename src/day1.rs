use std::fs;

fn split_strings_from_file_and_sort() -> (Vec<i32>, Vec<i32>) {
    let contents =
        fs::read_to_string("src/inputs/day1.txt").expect("Something went wrong reading the file");

    let split: Vec<&str> = contents.split("\n").collect();

    let mut list_one = Vec::new();
    let mut list_two = Vec::new();

    for i in split {
        let values: Vec<&str> = i.split_whitespace().collect();
        list_one.push(str::parse::<i32>(values[0]).unwrap());
        list_two.push(str::parse::<i32>(values[1]).unwrap());
    }

    list_one.sort();
    list_two.sort();

    (list_one, list_two)
}

pub fn solve_part_1() {
    let (list_one, list_two) = split_strings_from_file_and_sort();

    let mut total = 0;

    for (indx, item) in list_one.iter().enumerate() {
        total += (item - list_two[indx]).abs();
    }

    println!("total: {}", total);
}

pub fn solve_part_2() {
    let (list_one, list_two) = split_strings_from_file_and_sort();

    let mut items: Vec<Vec<i32>> = Vec::new();

    for item in list_one {
        items.push(vec![
            item,
            list_two.iter().filter(|&x| *x == item).count() as i32,
        ]);
    }

    let mut total = 0;

    for arr in items {
        total += arr[0] * arr[1];
    }

    println!("total: {}", total);
}
