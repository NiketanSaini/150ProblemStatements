fn main() {
    //Example 1
    let mut nums = vec!(3, 2, 2, 3);
    let val = 3;
    let result = remove_element(&mut nums, val);
    println!("{:?} {:?}", result, nums); 

    //Example 2
    let mut nums = vec!(0,1,2,2,3,0,4,2);
    let val = 2;
    let result = remove_element(&mut nums, val);
    println!("{:?} {:?}", result, nums);

    //Example 3
    let mut nums = vec!(0,2,3,1,1,2,0,2,4);
    let val = 99;
    let result = remove_element(&mut nums, val);
    println!("{:?} {:?}", result, nums);
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    if val > 50 {
        return nums.len().try_into().unwrap();
    }
    for i in 0..nums.len() {
        if nums[i]!=val {
            nums[index] = nums[i];
            index += 1;
        }
    }
    return index.try_into().unwrap();
}
