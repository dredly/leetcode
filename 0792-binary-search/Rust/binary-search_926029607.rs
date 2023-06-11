use std::cmp::Ordering;
use std::convert::TryInto;

fn search_with_offset(nums: Vec<i32>, target: i32, offset: usize) -> i32 {
    match nums.len() {
        1 => if nums[0] == target { offset.try_into().unwrap() } else { -1 },
        _ => {
            let middle_idx = nums.len() / 2;
            let middle_val = nums[middle_idx];
            match middle_val.cmp(&target) {
                Ordering::Less => {
                    let right = &nums[middle_idx + 1 ..];
                    let new_offset = offset + middle_idx + 1;
                    if right.len() == 0 { -1 } else {
                        search_with_offset(right.to_vec(), target, new_offset)
                    }
                },
                Ordering::Greater => {
                    let left = &nums[..middle_idx];
                    if left.len() == 0 { -1 } else {
                        search_with_offset(left.to_vec(), target, offset)
                    }
                },
                Ordering::Equal => (offset + middle_idx).try_into().unwrap()
            }
        }
    }
}

impl Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        search_with_offset(nums, target, 0)
    }
}