# Merge Sorted Arrays

## Problem Statement
You are given two sorted arrays of integers, `nums1` and `nums2`, and an integer `m` representing the number of elements in `nums1` and `n` representing the number of elements in `nums2`. Merge `nums2` into `nums1` as one sorted array.

The final sorted array should not use any additional space; it should be modified in-place.

### Constraints:
- `nums1.length == m + n`
- `nums2.length == n`
- `0 ≤ m, n ≤ 200`
- `1 ≤ m + n ≤ 200`
- `-10^9 ≤ nums1[i], nums2[i] ≤ 10^9`

### Example:
#### Input:
```plaintext
nums1 = [1, 2, 3, 0, 0, 0], m = 3
nums2 = [2, 5, 6], n = 3
```

#### Output:
```plaintext
nums1 = [1, 2, 2, 3, 5, 6]
```

### Explanation:
The first three elements of `nums1` are `[1, 2, 3]`, and the `0`s are placeholders for the additional elements from `nums2`. After merging, the array becomes `[1, 2, 2, 3, 5, 6]`.

## Tasks:
1. Implement a solution in **TypeScript**, **Go**, and **Rust**.
2. Ensure that the solution modifies `nums1` in-place without using extra space.

## Tips:
- Start merging from the end of the arrays to avoid overwriting elements in `nums1`.
- Use a pointer for the end of the merged array and decrement it as elements are added.

## File Structure:
```
mergeSortedArray/
├── ts/
│   ├── solution.ts
├── go/
│   ├── main.go
├── rust/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── README.md
```

