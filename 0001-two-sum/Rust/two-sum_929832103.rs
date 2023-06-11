use std::convert::TryInto;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_length: i32 = nums.len().try_into().unwrap();
        if nums_length == 2 {
            return vec![0, 1];
        }
        let mut i: i32 = 0;
        while i < nums_length {
            let mut j = i + 1;
            while j < nums_length {
                if nums[i as usize] + nums[j as usize] == target {
                    return vec![i, j];
                }
                j = j + 1;
            }
            i = i + 1;
        }
        panic!("Did not find two sum")       
    }
}