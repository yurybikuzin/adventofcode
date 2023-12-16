#[derive(Default)]
pub struct CalibrationValuesSummator {
    first_digit: Option<u8>,
    last_digit: Option<u8>,
    accu: u64,
}

impl CalibrationValuesSummator {
    pub fn feed(&mut self, chunk: &[u8]) {
        for b in chunk {
            match *b {
                CR_CODE | NL_CODE => {
                    self.eol();
                }
                b @ ZERO_CODE..=NINE_CODE => {
                    let value = b - ZERO_CODE;
                    if self.first_digit.is_none() {
                        self.first_digit = Some(value);
                    } else {
                        self.last_digit = Some(value);
                    }
                }
                _ => {}
            }
        }
    }
    pub fn finish(mut self) -> u64 {
        self.eol();
        self.accu
    }
    fn eol(&mut self) {
        if let Some(tens) = self.first_digit.take() {
            let units = self.last_digit.take().unwrap_or(tens);
            self.accu += 10 * tens as u64 + units as u64;
        }
    }
}

const ZERO_CODE: u8 = 0x30;
const NINE_CODE: u8 = 0x39;
const CR_CODE: u8 = 0xd;
const NL_CODE: u8 = 0xa;
