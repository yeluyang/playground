/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ret = Vec::new();
        if !nums.is_empty() && nums[0] <= 0 {
            let mut i = 0usize;
            while i < nums.len() {
                let t = -nums[i];
                let mut l = i + 1;
                let mut r = nums.len() - 1;
                while l < r {
                    let s = nums[l] + nums[r];
                    if s == t {
                        ret.push(vec![nums[i], nums[l], nums[r]]);
                        while l + 1 < nums.len() && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        l += 1;
                        while r > 0 && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        if r != 0 {
                            r -= 1;
                        }
                    } else if s < t {
                        l += 1;
                    } else if r != 0 {
                        r -= 1;
                    }
                }
                while i + 1 < nums.len() && nums[i] == nums[i + 1] {
                    i += 1;
                }
                i += 1;
            }
        }
        ret
    }
}
// @lc code=end
