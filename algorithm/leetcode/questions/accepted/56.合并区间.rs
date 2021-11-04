/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

// @lc code=start
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|v| v[0]);
        let mut ret = Vec::new();
        let mut last: Option<Vec<i32>> = None;
        for i in intervals {
            if let Some(l) = last {
                if (l[0] <= i[0] && i[0] <= l[1])
                    || (l[0] <= i[1] && i[1] <= l[1])
                    || (i[0] <= l[0] && l[1] <= i[1])
                {
                    last = Some(vec![std::cmp::min(l[0], i[0]), std::cmp::max(l[1], i[1])]);
                } else {
                    ret.push(l);
                    last = Some(i);
                }
            } else {
                last = Some(i);
            }
        }
        if let Some(l) = last {
            ret.push(l);
        }
        ret
    }
}
// @lc code=end
/* [note] 一遍过，但我想到还有更高级更快的算法有待验证：

使用平衡二叉搜索树，树节点包含区间起止点。插入节点时：

- 不重合区间按照二叉搜索树的顺序，低位区间向左，高位区间向右
- 重合的区间直接更新节点，并且检查更新后是否能融合左右子树根

该算法复杂度：

- 时间复杂度：O(TODO)
- 空间复杂度：O(TODO)
*/
