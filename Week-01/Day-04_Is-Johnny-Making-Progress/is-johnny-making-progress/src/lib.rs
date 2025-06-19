use std::num::{ParseIntError};

pub fn progress_days(inp: Vec<i64>) -> i64 {
    let mut result = 0;
    let len = if inp.len() > 0 { inp.len() - 1 } else { 0 };

    for i in 0..len {
        if inp[i] < inp[i + 1] {
            result += 1;
        }
    }
    result
}

pub fn parse_line_to_result_vec(line: &str) -> Result<Vec<i64>, ParseIntError> {
    let vec = parse_line_to_vec_int(line).iter()
        .filter(|x| { !x.trim().eq_ignore_ascii_case("") })
        .cloned()
        .collect();
    parse_vec_str_to_vec_int(vec)
}

fn parse_line_to_vec_int(buffer: &str) -> Vec<&str> {
    Vec::from_iter(buffer.split(' '))
}

fn parse_vec_str_to_vec_int(buffer: Vec<&str>) -> Result<Vec<i64>, ParseIntError> {
    let mut vec: Vec<i64> = Vec::new();

    for item in buffer {
        match parse_str_to_int(item.trim()) {
            Ok(n) => { vec.push(n); },
            Err(e) => { return Err(e); }
        }
    }
    Ok(vec)
}

fn parse_str_to_int(s: &str) -> Result<i64, ParseIntError> {
    match s.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::progress_days;
    use crate::parse_line_to_result_vec;
    use crate::parse_line_to_vec_int;
    use crate::parse_vec_str_to_vec_int;
    use crate::parse_str_to_int;

    #[test]
    fn test_progress_days_empty() {
        assert_eq!(progress_days(vec![]), 0);
    }

    #[test]
    fn test_progress_days_one() {
        assert_eq!(progress_days(vec![0]), 0);
    }

    #[test]
    fn test_progress_days_case1() {
        assert_eq!(progress_days(vec![3, 4, 1, 2]), 2);
    }

    #[test]
    fn test_progress_days_case2() {
        assert_eq!(progress_days(vec![10, 11, 12, 9, 10]), 3);
    }

    #[test]
    fn test_progress_days_case3() {
        assert_eq!(progress_days(vec![6, 5, 4, 3, 2, 9]), 1);
    }

    #[test]
    fn test_progress_days_case4() {
        assert_eq!(progress_days(vec![9, 9]), 0);
    }

    #[test]
    fn test_parse_line_to_result_vec_empty() {
        let result = parse_line_to_result_vec("");
        println!("test_parse_line_to_result_vec_empty | result: {:?}", result);
    }

    #[test]
    fn test_parse_line_to_result_vec_one() {
        let result = parse_line_to_result_vec("1");
        println!("test_parse_line_to_result_vec_empty | result: {:?}", result);
    }

    #[test]
    fn test_parse_line_to_result_vec_case1() {
        let result = parse_line_to_result_vec("3 4 1 2");
        println!("test_parse_line_to_result_vec_empty | result: {:?}", result);
    }

    #[test]
    fn test_parse_line_to_result_vec_case2() {
        assert_eq!(parse_line_to_result_vec("3 4 1 2 "), Ok(vec![3, 4, 1, 2]));
    }

    #[test]
    fn test_parse_line_to_vec_int_empty() {
        assert_eq!(parse_line_to_vec_int(""), vec![""]);
    }

    #[test]
    fn test_parse_line_to_vec_int_one() {
        assert_eq!(parse_line_to_vec_int("1"), vec!["1"]);
    }

    #[test]
    fn test_parse_line_to_vec_int_case1() {
        assert_eq!(parse_line_to_vec_int("3 4 1 2"), vec!["3", "4", "1", "2"]);
    }

    #[test]
    fn test_parse_vec_str_to_vec_int_empty() {
        let result = parse_vec_str_to_vec_int(vec![""]);
        println!("{:?}", result);
    }

    #[test]
    fn test_parse_vec_str_to_vec_int_one() {
        assert_eq!(parse_vec_str_to_vec_int(vec!["1"]), Ok(vec![1]));
    }

    #[test]
    fn test_parse_str_to_int_empty() {
        let result = parse_str_to_int("");
        println!("{:?}", result);
    }

    #[test]
    fn test_parse_str_to_int_one() {
        assert_eq!(parse_str_to_int("1"), Ok(1));
    }
}
