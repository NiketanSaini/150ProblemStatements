fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;

    merge(&mut nums1, m, &nums2, n);
    println!("{:?}", nums1); // Output: [1, 2, 2, 3, 5, 6]

    let mut nums3 = vec![1];
    let nums4: Vec<i32> = vec![];
    let m = 1;
    let n = 0;

    merge(&mut nums3, m, &nums4, n);
    println!("{:?}", nums3); // Output: [1]

    let mut nums5 = vec![0];
    let nums6 = vec![1];
    let m = 0;
    let n = 1;

    merge(&mut nums5, m, &nums6, n);
    println!("{:?}", nums5); // Output: [1]
}

fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &[i32], n: usize) {
    let mut p1 = m as isize - 1; // Pointer for nums1
    let mut p2 = n as isize - 1; // Pointer for nums2
    let mut p = (m + n) as isize - 1; // Pointer for the merged array

    while p2 >= 0 {
        if p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
            nums1[p as usize] = nums1[p1 as usize];
            p1 -= 1;
        } else {
            nums1[p as usize] = nums2[p2 as usize];
            p2 -= 1;
        }
        p -= 1;
    }
}
