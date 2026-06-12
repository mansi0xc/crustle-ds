pub fn bsearch(nums: &[i64], target: i64) -> i64 {
    let mut start = 0;
    let mut end = nums.len();
    println!("End = {end}");

    while start < end {
        let mid = start + (end - start) / 2;

        if nums[mid] == target {
            return mid as i64;
        }

        if nums[mid] > target {
            end = mid;
        }
        if nums[mid] < target {
            start = mid + 1;
        }
    }
    return -1 as i64;
}