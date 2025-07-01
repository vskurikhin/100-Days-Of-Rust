pub fn recursive_bst_count(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    let mut result = 0;
    for i in 1..n + 1 {
        result += recursive_bst_count(i - 1) * recursive_bst_count(n - i);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::recursive_bst_count;

    #[test]
    fn empty() {}

    #[test]
    fn recursive_bst_count_test_case_0() {
        assert_eq!(recursive_bst_count(0), 1);
    }

    #[test]
    fn recursive_bst_count_test_case_1() {
        assert_eq!(recursive_bst_count(1), 1);
    }

    #[test]
    fn recursive_bst_count_test_case_2() {
        assert_eq!(recursive_bst_count(2), 2);
    }

    #[test]
    fn recursive_bst_count_test_case_3() {
        assert_eq!(recursive_bst_count(3), 5);
    }

    #[test]
    fn recursive_bst_count_test_case_19() {
        assert_eq!(recursive_bst_count(19), 1767263190);
    }
}
