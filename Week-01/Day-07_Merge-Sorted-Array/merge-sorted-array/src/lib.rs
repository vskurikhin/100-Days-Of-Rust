extern crate core;

pub use num_traits::*;
use rand;
pub use rand::Rng;
pub use rand::SeedableRng;
pub use rand_isaac::IsaacRng;

pub fn merge_sorted<'a>(nums1: &'a mut [i32], nums2: &[i32], m: usize, n: usize) -> &'a [i32] {
    let mut vec = Vec::with_capacity(m);
    for i in 0..m {
        vec.push(nums1[i]);
    }
    let bs: Box<[i32]> = vec.into_boxed_slice();
    if !merge_sort_arrays(nums1, bs.iter().as_slice(), nums2) {
        panic!(
            "wrong size of slices: nums1.len(): {}, nums2.len(): {} or m: {}, n: {}",
            nums1.len(),
            nums2.len(),
            m,
            n
        )
    }
    nums1
}

pub fn merge_sort_arrays(result: &mut [i32], array1: &[i32], array2: &[i32]) -> bool {
    if result.len() != array1.len() + array2.len() {
        return false;
    }
    let v = _merge_sort(&Vec::from(array1), &Vec::from(array2));
    for i in 0..v.len() {
        result[i] = v[i];
    }
    true
}

pub fn merge_sort_arrays_to_vec(array1: &[i32], array2: &[i32]) -> Vec<i32> {
    _merge_sort(&Vec::from(array1), &Vec::from(array2))
}

pub fn merge_sort_vec(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() == 1 {
        return vec.clone();
    }
    let split_index = vec.len() / 2;
    let (left, right) = vec.split_at(split_index);

    _merge_sort(&mut Vec::from(left), &mut Vec::from(right))
}

fn _merge_sort(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    if vec1.len() == 0 {
        if vec2.len() == 0 {
            return vec![];
        }
        return merge_sort_vec(vec2);
    } else if vec2.len() == 0 {
        return merge_sort_vec(vec1);
    }
    let left = merge_sort_vec(vec1);
    let right = merge_sort_vec(vec2);
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

pub fn merge_sort<T: Signed + Clone + PartialOrd>(array: &mut [T]) -> &[T] {
    if array.len() <= 1 {
        return array; // База рекурсии: если 1 элемент или пустой массив — сортировать не нужно
    }
    let mid = array.len() / 2;
    let mut left = array[..mid].to_vec(); // Копируем левую половину
    let mut right = array[mid..].to_vec(); // Копируем правую половину

    merge_sort(&mut left); // Рекурсивная сортировка левой части
    merge_sort(&mut right); // Рекурсивная сортировка правой части

    merge(array, &left, &right); // Объединяем отсортированные части
    array
}

fn merge<T: Signed + Clone + PartialOrd>(array: &mut [T], left: &[T], right: &[T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    // Пока в обеих половинах есть элементы, выбираем минимальный и добавляем в array
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            array[k] = left[i].clone();
            i += 1;
        } else {
            array[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Добавляем оставшиеся элементы из левой части (если есть)
    while i < left.len() {
        array[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    // Добавляем оставшиеся элементы из правой части (если есть)
    while j < right.len() {
        array[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_empty_case() {}

    #[test]
    fn test_merge_sorted_case_1() {
        let mut array1 = [1, 2, 3, 0, 0, 0];
        let array2 = [2, 5, 6];
        let result = merge_sorted(&mut array1, &array2, 3, 3);
        assert_eq!(result, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    #[should_panic]
    fn test_merge_sorted_case_negative() {
        let mut array1 = [1, 2, 3];
        let array2 = [2, 5, 6];
        merge_sorted(&mut array1, &array2, 3, 3);
    }

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
        let mut result = [];
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

    #[test]
    fn test_merge_sort_slice_case_0() {
        let mut array1: [i32; 0] = [];
        let result = merge_sort(&mut array1);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_merge_sort_slice_case_1() {
        let mut array1 = [1];
        let result = merge_sort(&mut array1);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_merge_sort_slice_case_2() {
        let mut array1 = [2, 1];
        let result = merge_sort(&mut array1);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_merge_sort_slice_case_2_ext() {
        let mut array1: [i128; 2] = [i128::MAX, i128::MIN];
        let result = merge_sort(&mut array1);
        assert_eq!(result, vec![i128::MIN, i128::MAX]);
    }

    #[test]
    fn test_merge_sort_slice_case_3() {
        let mut array1 = [1, 2, 3];
        let result = merge_sort(&mut array1);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sort_slice_case_3_reverse() {
        let mut array1 = [3, 2, 1];
        let result = merge_sort(&mut array1);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sort_slice_case_3_() {
        let mut array1 = [2, 3, 1];
        let result = merge_sort(&mut array1);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sort_case_20() {
        let mut array1 = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ];
        let result = merge_sort(&mut array1);
        assert_eq!(
            result,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
            ]
        );
    }

    #[test]
    fn test_merge_sort_reverse_case_20() {
        let mut array1 = [
            19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
        ];
        let result = merge_sort(&mut array1);
        assert_eq!(
            result,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
            ]
        );
    }

    fn overflow() {
        let mut s1 = IsaacRng::random::<[i128; u16::MAX as usize]>(&mut IsaacRng::seed_from_u64(6));
        let result = merge_sort(&mut s1);
        for i in 0..result.len() - 1 {
            assert!(result[i] <= result[i + 1]);
        }
    }

    #[test]
    fn test_merge_sort_case_random() {
        const N: usize = u16::MAX as usize;
        const STACK_SIZE: usize = size_of::<i128>() * (N as f64 * 4.985) as usize;
        println!(
            "size of array: {}, size of stack: {}",
            size_of::<i128>() * N,
            STACK_SIZE
        ); // 1_048_560 5_242_800
        std::thread::Builder::new()
            .stack_size(STACK_SIZE) // env RUST_MIN_STACK
            .spawn(|| overflow())
            .unwrap()
            .join()
            .unwrap();
    }
}
