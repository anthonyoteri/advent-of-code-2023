fn part_1(input: &str) -> u32 {
    let values: Vec<u32> = input
        .lines()
        .into_iter()
        .map(|line| {
            let numbers: Vec<u8> = line
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            let sum: u32 = numbers[0] as u32 * 10 + *numbers.last().unwrap() as u32;
            sum
        })
        .collect();
    values.iter().sum()
}

fn part_2(input: &str) -> u32 {
    let values: Vec<u32> = input
        .lines()
        .into_iter()
        .map(|line| {
            let before = line.clone();

            let line = line
                .to_owned()
                .replace("twoneighthree", "2183")
                .replace("twoneightwo", "2182")
                .replace("oneighthree", "182")
                .replace("oneightwo", "182")
                .replace("twoneight", "218")
                .replace("oneight", "18")
                .replace("twone", "21")
                .replace("threeightwo", "382")
                .replace("threeighthree", "383")
                .replace("threeight", "38")
                .replace("fiveighthree", "583")
                .replace("fiveightwo", "582")
                .replace("fiveight", "58")
                .replace("eighthree", "83")
                .replace("eightwo", "82")
                .replace("sevenineightwo", "7982")
                .replace("sevenineighthree", "7983")
                .replace("sevenineight", "798")
                .replace("sevenine", "79")
                .replace("nineight", "98")
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9");

            let numbers: Vec<u8> = line
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            let sum: u32 = numbers[0] as u32 * 10 + *numbers.last().unwrap() as u32;
            println!("{before} -> {line} => {numbers:?} => {sum}");
            assert!(sum >= 10 && sum <= 99);
            sum
        })
        .collect();
    values.iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let part_1_output = part_1(input);
    let part_2_output = part_2(input);

    dbg!(part_1_output);
    dbg!(part_2_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../test_input_part_1.txt");

        let output = part_1(input);
        assert_eq!(output, 142);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../test_input_part_2.txt");

        let output = part_2(input);
        assert_eq!(output, 281);
    }
}
