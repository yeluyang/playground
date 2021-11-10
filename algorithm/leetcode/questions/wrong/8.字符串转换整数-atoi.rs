/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let cs: Vec<char> = s.chars().collect();
        let mut sign = None;
        let mut ret = 0i32;
        for c in &cs {
            if !c.is_ascii_digit() {
                if sign.is_some() {
                    break;
                } else if *c == ' ' {
                    continue;
                } else if *c == '-' {
                    sign = Some(-1);
                } else if *c == '+' {
                    sign = Some(1);
                } else {
                    break;
                }
            } else {
                if sign.is_none() {
                    sign = Some(1);
                }
                let t = (*c as u8 - b'0') as i32;
                if ret > i32::MAX / 10 || (ret == i32::MAX / 10 && t > i32::MAX % 10) {
                    if let Some(sign) = sign {
                        if sign == -1 {
                            return i32::MIN;
                        } else {
                            return i32::MAX;
                        }
                    } else {
                        return i32::MAX;
                    }
                } else {
                    ret = ret * 10 + t;
                }
            }
        }
        if let Some(sign) = sign {
            ret *= sign;
        }
        return ret;
    }
}
// @lc code=end

// [wrong] 2021/10/08-1 溢出处理没做出来。还改了很多遍，工业代码能力不够
