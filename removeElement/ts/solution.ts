//Example 1
let nums: number[] = [3,2,2,3]
let val: number = 3
let result = removeElement(nums, val)
console.log(result, nums)

//Example 2
nums = [0,1,2,2,3,0,4,2]
val = 2
result = removeElement(nums, val)
console.log(result, nums)

//Example 3
nums = [0,2,3,1,1,2,0,2,4]
val = 99
result = removeElement(nums, val)
console.log(result, nums)

function removeElement(nums: number[], val: number): number{
    let index: number = 0;
    if(val > 50) return nums.length;
    for(let i: number = 0; i < nums.length; i++){
        if(nums[i]!= val){
            nums[index] = nums[i]
            index++
        }
    }
    return index;
}