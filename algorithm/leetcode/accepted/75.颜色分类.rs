/*
 * @lc app=leetcode.cn id=75 lang=rust
 *
 * [75] 颜色分类
 */

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut zero = 0usize;
        let mut one = 0usize;
        // [0..zero) is all zero
        // [zero..one) is all one
        // [one..i) is all two
        for i in 0..nums.len() {
            match nums[i] {
                0 => {
                    nums[zero] = 0;
                    if zero != one {
                        nums[one] = 1;
                    }
                    if one != i {
                        nums[i] = 2;
                    }
                    zero += 1;
                    one += 1;
                }
                1 => {
                    nums[one] = 1;
                    if one != i {
                        nums[i] = 2;
                    }
                    one += 1;
                }
                2 => {}
                _ => unreachable!(),
            }
        }
    }
}
// @lc code=end
