package main

import "fmt" 

func main(){
	nums1 := []int{1, 2, 3, 0, 0, 0}
	nums2 := []int{2, 5, 6}
	m,n := 3,3

	merge(nums1, m, nums2, n);
	fmt.Println(nums1);

	nums1 = []int{1}
	nums2 = []int{}
	m,n = 1,0

	merge(nums1, m, nums2, n);
	fmt.Println(nums1);

	nums1 = []int{0}
	nums2 = []int{1}
	m,n = 0,1

	merge(nums1, m, nums2, n);
	fmt.Println(nums1);
}

func merge(nums1 []int, m int, nums2 []int, n int) {
	p1, p2, p := m-1, n-1, m+n-1;

	for p2 >= 0 {
		if p1 >= 0 && nums1[p1] > nums2[p2] {
            nums1[p] = nums1[p1]
            p1--
        } else {
            nums1[p] = nums2[p2]
            p2--
        }
        p--
	}
}