/*
 * @lc app=leetcode.cn id=38 lang=rust
 *
 * [38] 外观数列
 */

// @lc code=start
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_owned();
        }
        let x = Self::count_and_say(n - 1);
        let b = x.as_bytes();
        let mut r = Vec::new();
        let mut count = 1;
        for i in 0..b.len() {
            if i + 1 < b.len() && b[i] == b[i + 1] {
                count += 1;
            } else {
                r.push(char::from_digit(count, 10).unwrap());
                r.push(b[i] as char);
                count = 1;
            }
        }
        r.iter().collect()
    }
}
// @lc code=end
