use crate::error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(0, process(input)?);
        Ok(())
    }
}
