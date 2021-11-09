/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 */

// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut amounts = vec![None; amount as usize + 1];
        amounts[0] = Some(0);

        for i in 1..amounts.len() {
            for coin in coins.iter() {
                if i >= *coin as usize {
                    let j = i as i32 - coin;
                    if let Some(pre) = &amounts[j as usize] {
                        if let Some(cur) = &amounts[i] {
                            if *pre + 1 < *cur {
                                amounts[i] = Some(pre + 1);
                            }
                        } else {
                            amounts[i] = Some(pre + 1);
                        }
                    }
                }
            }
        }
        match amounts.last().unwrap() {
            None => -1,
            Some(i) => *i,
        }
    }
}
// @lc code=end
