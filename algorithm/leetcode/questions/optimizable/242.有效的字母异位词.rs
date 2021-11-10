/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] 有效的字母异位词
 */

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {}
}
// @lc code=end

/* [optimizable] 2021/10/08-1 可以写的更简洁更可读
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut m = std::collections::HashMap::new();
    for c in s.chars() {
        *m.entry(c).or_insert(0) += 1;
    }
    for c in t.chars() {
        match m.get_mut(&c) {
            None => return false,
            Some(n) => {
                *n -= 1;
                if *n == 0 {
                    m.remove(&c);
                };
            }
        };
    }
    m.len() == 0
}
*/
