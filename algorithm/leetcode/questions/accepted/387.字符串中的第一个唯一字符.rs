/*
 * @lc app=leetcode.cn id=387 lang=rust
 *
 * [387] 字符串中的第一个唯一字符
 */

// @lc code=start
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut m = std::collections::HashMap::new();
        for b in s.as_bytes() {
            *m.entry(b).or_insert(0) += 1;
        }
        for (i, b) in s.as_bytes().iter().enumerate() {
            if *m.entry(b).or_insert(0) == 1 {
                return i as i32;
            }
        }
        -1
    }
}
// @lc code=end
