/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {}
}
// @lc code=end

/* [optimizable] 2021/10/08 还有另一种解法没有想到
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_owned();
    } else if strs.len() == 1 {
        return strs[0].clone();
    }
    let mut p = strs[0].as_bytes();
    for i in 1..strs.len() {
        if p.len() > strs[i].len() {
            p = &p[..strs[i].len()]
        }
        if p.is_empty() {
            return "".to_owned();
        }
        for (j, c) in strs[i].as_bytes().iter().enumerate() {
            if j >= p.len() {
                break;
            }
            if p[j] != *c {
                p = &p[..j];
                break;
            }
        }
    }
    return String::from_utf8(p.to_owned()).unwrap();
}
*/
