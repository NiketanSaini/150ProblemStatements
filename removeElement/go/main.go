package main

import "fmt"

func main(){
	// Example 1
	nums := []int{3, 2, 2, 3}
	val := 3
	result := removeElement(nums, val)
	fmt.Println(result, nums)

	// Example 2
	nums = []int{0,1,2,2,3,0,4,2}
	val = 2
	result = removeElement(nums, val)
	fmt.Println(result, nums)

	// Example 3
	nums = []int{0,2,3,1,1,2,0,2,4}
	val = 99
	result = removeElement(nums, val)
	fmt.Println(result, nums)

}

func removeElement(nums []int, val int) int {
	index := 0;
	if val > 50 {
		return len(nums)
	}
	for i := 0; i < len(nums); i++ {
		if nums[i] != val {
            nums[index] = nums[i]
            index++
        }
	}
	return index
}