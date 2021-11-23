/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.into_bytes();
        let mut l = 0usize;

        let mut m = std::collections::HashMap::new();
        let mut start = 0usize;
        let mut end = 0usize;
        while end < s.len() {
            match m.get(&s[end]) {
                Some(v) => {
                    if end - start > l {
                        l = end - start;
                    }
                    let tmp = *v + 1;
                    for i in start..tmp {
                        m.remove(&s[i]);
                    }
                    start = tmp;
                }
                None => {
                    m.insert(s[end], end);
                    end += 1;
                }
            };
        }

        if end - start > l {
            (end - start) as i32
        } else {
            l as i32
        }
    }
}
// @lc code=end
