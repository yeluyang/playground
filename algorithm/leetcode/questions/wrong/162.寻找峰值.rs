/*
 * @lc app=leetcode.cn id=162 lang=rust
 *
 * [162] 寻找峰值
 */

// @lc code=start
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = nums.len();
        while start < end {
            let mid = start + (end - start) / 2;
            if (mid <= 0 || nums[mid - 1] < nums[mid])
                && (mid >= nums.len() - 1 || nums[mid] > nums[mid + 1])
            {
                return mid as i32;
            } else if mid < nums.len() - 1 && nums[mid] <= nums[mid + 1] {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        unreachable!()
    }
}
// @lc code=end

// [wrong] 2021/11/03-1 没做出来，审题不清，重要条件没有用上，也没有耐心地深入分析
