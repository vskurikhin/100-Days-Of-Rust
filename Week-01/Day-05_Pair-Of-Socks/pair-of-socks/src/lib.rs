use std::collections::HashMap;

pub fn sock_pairs(line: &str) -> u64 {
    let mut result = 0u64;
    let mut socks_scores = HashMap::new();

    for char in line.to_string().chars() {
        if 'A' <= char && char <= 'Z' {
            let count = socks_scores.entry(char).or_insert(0u64);
            *count += 1;
        }
    }
    for (_, value) in &socks_scores {
        result += value / 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::sock_pairs;

    #[test]
    fn test_empty_case() {
        assert_eq!(sock_pairs(""), 0);
    }

    #[test]
    fn test_case_1() {
        assert_eq!(sock_pairs("AA"), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sock_pairs("ABABC"), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(sock_pairs("CABBACCC"), 4);
    }
}