use std::collections::BinaryHeap;

fn input(test: bool) -> &'static str {
    if test {
        include_str!("test_input/day01.txt")
    } else {
        include_str!("real_input/day01.txt")
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut elves = Vec::new();
    for elf in input.split("\n\n") {
        let mut total_calories = 0;
        for line in elf.split('\n') {
            let calories: usize = line
                .trim()
                .parse()
                .expect("Could not parse calorie value from string");
            total_calories += calories;
        }
        elves.push(total_calories);
    }
    elves
}

fn get_max_n(i: &[usize], n: usize) -> Vec<usize> {
    let mut max_heap = BinaryHeap::new();
    for _i in i {
        max_heap.push(_i);
    }
    let mut result: Vec<usize> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let top = max_heap.pop().expect("Tried to pop from an empty max heap");
        result.push(*top);
    }
    result
}

pub fn a(test: bool) {
    let input = input(test);
    let elves = parse_input(input);
    let max_one = get_max_n(&elves, 1);
    let a_sum: usize = max_one.iter().sum();
    println!("Day 01: A = {a_sum}");
    let max_three = get_max_n(&elves, 3);
    let b_sum: usize = max_three.iter().sum();
    println!("Day 01: B = {b_sum}");
}
