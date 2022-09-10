fn main() {
    let day = 10;
    solve_day(&day);
}

fn solve_day(day: &i32) {
    match day {
        1 => solve_day_1(),
        2 => solve_day_2(),
        3 => solve_day_3(),
        4 => solve_day_4(),
        5 => solve_day_5(),
        6 => solve_day_6(),
        7 => solve_day_7(),
        8 => solve_day_8(),
        9 => solve_day_9(),
        10 => solve_day_10(),
        _ => println!("You didn't complete this day yet"),
    }
}

fn solve_day_1() {
    let file_name: &str = "inputDay1.txt";
    let target = 2020;
    let result_part_1: i32 = advent_of_code::day1::part1(&file_name, &target).unwrap();
    println!("The result of the first part is: {:?}", result_part_1);
    let result_part_2: i32 = advent_of_code::day1::part2(&file_name, &target).unwrap();
    println!("The result for the second part is {:?}", result_part_2);
}

fn solve_day_2() {
    let file_name = "inputDay2.txt";
    let result = advent_of_code::day2::part1(&file_name).unwrap();
    println!("The number of legal passwords is: {:?}", result);
    let result_2 = advent_of_code::day2::part2(&file_name).unwrap();
    println!("The number of legal passwords id {:?}", result_2);
}

fn solve_day_3() {
    let file_name = "inputDay3.txt";
    let right = 3;
    let down = 1;
    let result: i32 = advent_of_code::day3::part1(&file_name, &right, &down).unwrap();
    println!("Number of trees: {}", result);
    let final_result: u128 = advent_of_code::day3::part2(&file_name).unwrap();
    println!("Product of the 5 paths is {}", final_result);
}

fn solve_day_4() {
    let file_name = "inputDay4.txt";
    let result = advent_of_code::day4::part1(&file_name).unwrap();
    println!("The number of valid passports is {}", result);
    let result_2 = advent_of_code::day4::part2(&file_name).unwrap();
    println!("The new number of valid passports is {}", result_2);
}

fn solve_day_5() {
    let file_name = "inputDay5.txt";
    let result = advent_of_code::day5::part1(&file_name).unwrap();
    println!("Highest seat ID: {:?}", result);
    let result_2 = advent_of_code::day5::part2(&file_name).unwrap();
    println!("Your seat is: {}", result_2);
}

fn solve_day_6() {
    let file_name = "inputDay6.txt";
    let result = advent_of_code::day6::part1(&file_name).unwrap();
    println!("Total number of questions is: {:?}", result);
    let result_2 = advent_of_code::day6::part2(&file_name).unwrap();
    println!(
        "Sum of questions answered affermatively by each member of a given group: {}",
        result_2
    );
}

fn solve_day_7() {
    let file_name = "inputDay7.txt";
    let result = advent_of_code::day7::part1(&file_name).unwrap();
    println!(
        "Shiny gold bag can be contained in {} types of bags",
        result
    );
    let result_2 = advent_of_code::day7::part2(&file_name).unwrap();
    println!("Shiny gold bag must contain {} bags", result_2);
}

fn solve_day_8() {
    let file_name = "inputDay8.txt";
    let result = advent_of_code::day8::part1(file_name).unwrap();
    println!(
        "At the end of the first cycle, the accumulator leads to {}",
        result
    );
    let result_2 = advent_of_code::day8::part2(file_name).unwrap();
    println!("The accumulator of the fixed game leads to {}", result_2);
}

fn solve_day_9() {
    let file_name = "inputDay9.txt";
    let result = advent_of_code::day9::part1(&file_name).unwrap();
    println!("Faulty number is {}", result);
    let result_2 = advent_of_code::day9::part2(&file_name).unwrap();
    println!("Sum of the two pieces of faulty number is {}", result_2);
}

fn solve_day_10() {
    let file_name = "inputDay10.txt";
    let result = advent_of_code::day10::part1(file_name).unwrap();
    println!("Product betwneen #3-jumps and #1-jumps is {}", result);
    let result_2 = advent_of_code::day10::part2(file_name).unwrap();
    println!("Total number of independent legit paths {}", result_2);
}
