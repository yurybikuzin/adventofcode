pub mod one_liner;
pub mod summator;

#[cfg(test)]
mod tests {
    use super::*;

    const FOR_EXAMPLE: &str = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;

    #[test]
    fn test_one_liner() {
        assert_eq!(
            142,
            one_liner::sum_of_all_of_the_calibration_values(FOR_EXAMPLE)
        );
    }

    #[test]
    fn test_summator() {
        assert_eq!(142, {
            let mut summator = summator::CalibrationValuesSummator::default();
            summator.feed(FOR_EXAMPLE[..FOR_EXAMPLE.len() / 3].as_bytes());
            summator.feed(FOR_EXAMPLE[FOR_EXAMPLE.len() / 3..FOR_EXAMPLE.len() / 2].as_bytes());
            summator.feed(FOR_EXAMPLE[FOR_EXAMPLE.len() / 2..FOR_EXAMPLE.len()].as_bytes());
            summator.finish()
        });
    }
}
