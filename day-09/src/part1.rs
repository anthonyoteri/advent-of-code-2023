use crate::error::AocError;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_next(input: Vec<i32>) -> i32 {
    let input = input.clone();

    if input.iter().all(|&v| v == 0) {
        return 0;
    }

    let next_vec = input[1..]
        .iter()
        .enumerate()
        .map(|(i, num)| num - input[i])
        .collect::<Vec<_>>();

    input.last().unwrap() + get_next(next_vec.clone())
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i64, AocError> {
    let input = parse(input);
    Ok(input
        .iter()
        .map(|v| get_next(v.clone()))
        .map(i64::from)
        .sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_get_next() {
        let input = vec![-3, 0, 3, 6, 9, 12, 15];
        assert_eq!(get_next(input), 18);
        let input = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(get_next(input), 28);
        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(get_next(input), 68);
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(114, process(input)?);
        Ok(())
    }
}
