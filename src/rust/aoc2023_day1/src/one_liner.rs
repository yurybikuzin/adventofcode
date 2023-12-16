pub fn sum_of_all_of_the_calibration_values(s: &str) -> u64 {
    s.lines()
        .filter_map(|s| {
            s.find(DIGITS)
                .and_then(|idx| s.chars().nth(idx))
                .map(|ch| (ch as u32) - ZERO_CHAR_CODE)
                .and_then(|tens| {
                    s.rfind(DIGITS)
                        .and_then(|idx| s.chars().nth(idx))
                        .map(|ch| {
                            let units = (ch as u32) - ZERO_CHAR_CODE;
                            10 * tens as u64 + units as u64
                        })
                })
        })
        .sum::<u64>()
}

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const ZERO_CHAR_CODE: u32 = DIGITS[0] as u32;
