use std::convert::TryInto;
use std::str::from_utf8;

#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64)
}

impl Machine {
    fn from_raw(input: &str, offset: Option<u64>) -> Self {
        let lines: Vec<&str> = input.split("\r\n").collect();
        let (raw_a, raw_b, raw_prize) = (lines[0], lines[1], lines[2]);

        let a: Vec<u64> = raw_a[12..].split(", Y+").map(|s| s.parse().expect("Invalid digit in a")).collect();
        let b: Vec<u64> = raw_b[12..].split(", Y+").map(|s| s.parse().expect("Invalid digit in b")).collect();
        let prize: Vec<u64> = raw_prize[9..].split(", Y=").map(|s| s.parse().expect("Invalid digit in prize")).collect();

        let offset = offset.unwrap_or(0);

        Machine {
            a: (a[0], a[1]),
            b: (b[0], b[1]),
            prize: (prize[0] + offset, prize[1] + offset),
        }
    }

    fn min_tokens(&self) -> i64 {
        //debug!("{:?}", self);
        let prize0 = self.prize.0 as i128;
        let prize1 = self.prize.1 as i128;
        let a0 = self.a.0 as i128;
        let a1 = self.a.1 as i128;
        let b0 = self.b.0 as i128;
        let b1 = self.b.1 as i128;

        let top = (prize1 * a0 * b0) - (prize0 * a0 * b1);
        let bottom = (a1 * b0) - (a0 * b1);

        if bottom == 0 {
            return 0;
        }

        let x = top / bottom;

        if x % 1 != 0 {
            return 0;
        }

        if x % a0 == 0 && (prize0 - x) % b0 == 0 {
            let a_count = x / a0;
            let b_count = (prize0 - x) / b0;

            return (a_count * 3 + b_count).try_into().unwrap_or(0);
        }

        0
    }
}

pub fn part_1(input: &[u8]) -> i64 {
    from_utf8(input).unwrap().split("\r\n\r\n").map(|s| Machine::from_raw(s, None).min_tokens()).sum()
}

pub fn part_2(input: &[u8]) -> i64 {
    from_utf8(input).unwrap().split("\r\n\r\n").map(|s| Machine::from_raw(s, Some(10000000000000_u64)).min_tokens()).sum()
}