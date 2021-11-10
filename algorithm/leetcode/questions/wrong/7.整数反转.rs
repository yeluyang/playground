/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut n = x;
        let mut r = 0i32;
        while n != 0 {
            let t = n % 10;
            let nr = r * 10 + t;
            if ((nr - t) / 10) != r {
                return 0;
            }
            r = nr;
            n /= 10;
        }
        r
    }
}
// @lc code=end
