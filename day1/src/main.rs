use regex::Regex;


fn first_number(x: &str, re: Regex) -> &str {
    let checked = re.find(x);
    if checked.is_some() {
        return checked.unwrap().as_str()
    }
    "0"
}

fn last_number(x: &str, re: Regex) -> &str {
    // It's not as simple as reversing the string and applying a regex
    // It's also not as simple as matching everything and picking the last one
    // Some of the written numbers overlap. "eighthree" should be 3, not 8.
    // We build a string backwards and apply regex to each step
    for (i, _) in x.chars().enumerate() { // i = iteration number
        let l = x.len();
        let str_slice = &x[(l - i - 1)..l];
        let checked = re.find(str_slice);
        if checked.is_some() {
            return checked.unwrap().as_str()
        }
    }
    "0"
}

fn string_to_number(x: &str) -> u32 {
    match x {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse().unwrap()
    }
}

fn main() {
    let number_regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    let input = include_str!("input.txt");
    let mut output: Vec<u32> = [].to_vec();
    for (count, i) in input.lines().enumerate() {
        println!("Line {}: {}", count, i);
        let num1 = string_to_number(first_number(&i, number_regex.clone()));
        let num2 = string_to_number(last_number(&i, number_regex.clone()));
        let num_output: u32 = format!("{}{}", num1, num2).parse().unwrap();
        println!("{} and {}: {}", num1, num2, num_output);
        output.push(num_output);
    }
    println!("The sum of the inputs is: {}", output.iter().sum::<u32>());
}
