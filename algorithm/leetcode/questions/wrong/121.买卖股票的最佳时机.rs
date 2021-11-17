/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut dp0 = 0;
        let mut dp1 = -prices[0];
        for i in 1..prices.len() {
            dp0 = std::cmp::max(dp0, dp1 + prices[i]);
            dp1 = std::cmp::max(dp1, -prices[i]);
        }
        std::cmp::max(dp0, dp1)
    }
}
// @lc code=end
