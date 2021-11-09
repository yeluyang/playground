/*
 * @lc app=leetcode.cn id=217 lang=rust
 *
 * [217] 存在重复元素
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut m = std::collections::HashSet::new();
        for n in nums {
            if !m.insert(n) {
                return true;
            }
        }
        return false;
    }
}
// @lc code=end
