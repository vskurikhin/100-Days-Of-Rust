use rand;
pub use rand::Rng;
pub use rand_isaac::IsaacRng;
pub use rand::{SeedableRng};

pub fn merge_sort_arrays(result: &mut [i32], array1: &[i32], array2: &[i32]) -> bool {
    if result.len() != array1.len() + array2.len() {
        return false
    }
    let v = merge_sort(&mut Vec::from(array1), &mut Vec::from(array2));
    for i in 0..v.len() {
        result[i] = v[i];
    }
    true
}

pub fn merge_sort_arrays_to_vec(array1: &[i32], array2: &[i32]) -> Vec<i32> {
    merge_sort(&mut Vec::from(array1), &mut Vec::from(array2))
}

pub fn merge_sort(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) -> Vec<i32> {
    if vec1.len() == 0 {
        if vec2.len() == 0 {
            return vec![];
        }
        return _merge_sorted_helper(vec2);
    } else if vec2.len() == 0 {
        return _merge_sorted_helper(vec1);
    }
    let left = _merge_sorted_helper(vec1);
    let right = _merge_sorted_helper(vec2);
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    while i < left.len() || j < right.len() {
        if i == left.len() {
            result.push(right[j]);
            j += 1;
            continue;
        }
        if j == right.len() || left[i] < right[j] {
            result.push(left[i]);
            i += 1;
            continue;
        }
        result.push(right[j]);
        j += 1;
    }
    result
}

fn _merge_sorted_helper(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() == 1 {
        return vec.clone();
    }
    let split_index = vec.len() / 2;
    let (left, right) = vec.split_at(split_index);

    merge_sort(&mut Vec::from(left), &mut Vec::from(right))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_empty_case() {}

    #[test]
    fn test_test_case_0() {
        let array1 = [];
        let array2 = [];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_test_case_1() {
        let array1 = [0];
        let array2 = [];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_test_case_01() {
        let array1 = [];
        let array2 = [0];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_test_case_2() {
        let array1 = [0];
        let array2 = [1];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_test_case_22() {
        let array1 = [0];
        let array2 = [1];
        let mut result: [i32; 2] = [-1; 2];
        let ok = merge_sort_arrays(&mut result, &array1, &array2);
        assert!(ok);
        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn test_test_case_22_false() {
        let array1 = [0];
        let array2 = [1];
        let mut result= [];
        let ok = merge_sort_arrays(&mut result, &array1, &array2);
        assert!(!ok);
        assert_eq!(result, []);
    }

    #[test]
    fn test_test_case_02() {
        let array1 = [1];
        let array2 = [0];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_test_case_19() {
        let array1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array2 = [10, 11, 12, 13, 14, 15, 16, 17, 18];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(
            result,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18
            ]
        );
    }

    #[test]
    fn test_test_case_019() {
        let array1 = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let array2 = [9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(
            result,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18
            ]
        );
    }

    #[test]
    fn test_test_case_20() {
        let array1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array2 = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(
            result,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
            ]
        );
    }

    #[test]
    fn test_test_reverse_case_20() {
        let array1 = [19, 18, 17, 16, 15, 14, 13, 12, 11, 10];
        let array2 = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let result = merge_sort_arrays_to_vec(&array1, &array2);
        assert_eq!(
            result,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
            ]
        );
    }

    #[test]
    fn test_test_reverse_case_random() {
        let s1 = IsaacRng::random::<[i32; 65536]>(&mut IsaacRng::seed_from_u64(6));
        let s2 = IsaacRng::random::<[i32; 65536]>(&mut IsaacRng::seed_from_u64(6));
        let result = merge_sort_arrays_to_vec(&s1, &s2);
        for i in 0..result.len() - 1 {
            assert!(result[i] <= result[i + 1]);
        }
    }
}
