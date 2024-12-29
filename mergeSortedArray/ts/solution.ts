function merge(nums1: number[], m: number, nums2: number[], n: number): void {
    // Pointers for nums1, nums2, and the position to fill in nums1
    let p1: number = m - 1;
    let p2: number = n - 1;
    let p: number = m + n - 1;

    // Merge in reverse order
    while (p2 >= 0) {
        if (p1 >= 0 && nums1[p1] > nums2[p2]) {
            nums1[p] = nums1[p1];
            p1--;
        } else {
            nums1[p] = nums2[p2];
            p2--;
        }
        p--;
    }
}

// Example 1
let nums1 : number[] = [1, 2, 3, 0, 0, 0];
let m = 3;
let nums2 = [2, 5, 6];
let n = 3;
merge(nums1, m, nums2, n);
console.log(nums1); // Output: [1, 2, 2, 3, 5, 6]

// Example 2
nums1 = [1];
m = 1;
nums2 = [];
n = 0;
merge(nums1, m, nums2, n);
console.log(nums1); // Output: [1]

// Example 3
nums1 = [0];
m = 0;
nums2 = [1];
n = 1;
merge(nums1, m, nums2, n);
console.log(nums1); // Output: [1]
