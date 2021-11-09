/*
 * @lc app=leetcode.cn id=350 lang=rust
 *
 * [350] 两个数组的交集 II
 */

// @lc code=start
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (short, long) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };
        let mut m = std::collections::HashMap::new();
        for n in short {
            *m.entry(n).or_insert(0) += 1;
        }
        let mut v = Vec::new();
        for n in long {
            if let Some(count) = m.get_mut(n) {
                if *count > 0 {
                    *count -= 1;
                    v.push(*n);
                }
            }
        }
        return v;
    }
}
// @lc code=end
