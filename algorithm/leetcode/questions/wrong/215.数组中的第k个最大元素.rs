/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut topk = vec![i32::MIN; k as usize];
        for mut n in nums {
            for i in 0..k as usize {
                if n > topk[i] {
                    let tmp = topk[i];
                    topk[i] = n;
                    n = tmp;
                }
            }
        }
        *topk.last().unwrap()
    }
}
// @lc code=end

// [wrong] 没有做出最优解
