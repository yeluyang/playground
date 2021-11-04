/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut ret = -1;
        if nums[0] == target {
            ret = 0;
        } else {
            let mut start = 0usize;
            let mut end = nums.len();
            while start < end {
                let mid = start + (end - start) / 2;
                if nums[mid] == target {
                    ret = mid as i32;
                    break;
                } else if nums[0] < target {
                    if nums[0] < nums[mid] && nums[mid] < target {
                        start = mid + 1;
                    } else {
                        end = mid;
                    }
                } else {
                    if nums[mid] < nums[0] && target < nums[mid] {
                        end = mid;
                    } else {
                        start = mid + 1;
                    }
                }
            }
        }
        ret
    }
}
// @lc code=end
// [wrong] 2021/11/04-1 没有做出最优解
