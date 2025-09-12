use merge_sorted_array::merge_sorted;

fn main() {
    let mut nums1 = [1, 2, 3, 0, 0, 0];
    let mut nums2 = [2, 5, 6];

    merge_sorted(&mut nums1, &mut nums2, 3, 3);

    println!("{:?}", nums1);
}
