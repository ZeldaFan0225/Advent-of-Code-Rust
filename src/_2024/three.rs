static MATCHDONT: &[u8] = "don't()".as_bytes();
static MATCHDONTLENGTH: usize = MATCHDONT.len();
static MATCHDO: &[u8] = "do()".as_bytes();
static MATCHDOLENGTH: usize = MATCHDO.len();
static MATCHPATTERN: &[u8] = "mul(".as_bytes();
static MATCHPATTERNLENGTH: usize = MATCHPATTERN.len();

pub fn part_1(input: &[u8]) -> i64 {
    process_result(1, input).into()
}

pub fn part_2(input: &[u8]) -> i64 {
    process_result(2, input).into()
}

fn process_result(part: i32, input: &[u8]) -> u32 {
    let mut process = true;

    let mut dont_match_count = 0;
    let mut do_match_count = 0;

    let mut capture_numbers = false;
    let mut capture_match_count = 0;

    let mut left_num = 0;
    let mut right_num = 0;
    let mut write_right = false;

    let mut result: u32 = 0;

    for char in input {
        if *char == MATCHDONT[dont_match_count] {
            if dont_match_count == MATCHDONTLENGTH - 1 {
                process = false;
                dont_match_count = 0;
            } else {
                dont_match_count += 1;
            }
        } else {
            dont_match_count = 0;
        }

        if *char == MATCHDO[do_match_count] {
            if do_match_count == MATCHDOLENGTH - 1 {
                process = true;
                do_match_count = 0;
            } else {
                do_match_count += 1;
            }
        } else {
            do_match_count = 0;
        }

        if (process || part == 1) && *char == MATCHPATTERN[capture_match_count] {
            left_num = 0;
            right_num = 0;
            write_right = false;
            if capture_match_count == MATCHPATTERNLENGTH - 1 {
                capture_numbers = true;
                capture_match_count = 0;
                continue;
            } else {
                capture_match_count += 1;
            }
        } else {
            capture_match_count = 0;
        }

        if capture_numbers {
            match char {
                b',' => {
                    write_right = true;
                    continue;
                }
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    if write_right {
                        right_num = right_num * 10 + (char - b'0');
                    } else {
                        left_num = left_num * 10 + (char - b'0');
                    }
                }
                b')' => {
                    result += (left_num * right_num) as u32;
                    capture_numbers = false;
                    write_right = false;
                }
                _ => {
                    capture_numbers = false;
                    write_right = false;
                }
            }
        }
    }

    result
}