/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for i in 0..nums.len() {
            match m.get(&(target - nums[i])) {
                None => {
                    m.insert(nums[i], i as i32);
                }
                Some(j) => {
                    return vec![*j, i as i32];
                }
            }
        }
        unreachable!();
    }
}
// @lc code=end
