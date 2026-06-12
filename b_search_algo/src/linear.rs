pub fn search(nums: &[i64], target: i64) -> i64 {
    let start = 0;
    let end = nums.len();
    
    for i in start..end{
        if nums[i] == target {
            return i as i64
        }
    }
    return -1 as i64
}