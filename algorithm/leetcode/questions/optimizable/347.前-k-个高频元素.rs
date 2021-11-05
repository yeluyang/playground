/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] 前 K 个高频元素
 */

// @lc code=start
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = std::collections::HashMap::new();
        for n in nums {
            *count.entry(n).or_insert(0usize) += 1;
        }
        let mut buf = Vec::with_capacity(count.len());
        for (k, v) in count {
            buf.push((k, v));
        }
        let n = buf.len();
        quick_select(&mut buf, 0, n, k as usize)
    }
}

fn quick_select(n: &mut Vec<(i32, usize)>, left: usize, right: usize, k: usize) -> Vec<i32> {
    use rand::Rng;
    let i = quick_sort_step(n, rand::thread_rng().gen_range(left, right), left, right);
    if i < k - 1 {
        quick_select(n, i + 1, right, k)
    } else if i > k - 1 {
        quick_select(n, left, i, k)
    } else {
        let mut ret = Vec::with_capacity(k);
        for i in 0..k {
            ret.push(n[i].0);
        }
        ret
    }
}

fn quick_sort_step(n: &mut Vec<(i32, usize)>, guard: usize, left: usize, right: usize) -> usize {
    n.swap(right - 1, guard);
    let mut j = left;
    for i in left..right - 1 {
        if n[i].1 > n[right - 1].1 {
            n.swap(i, j);
            j += 1;
        }
    }
    n.swap(j, right - 1);
    j
}
// @lc code=end
// [optimizable] 2021/11/05-1 存在优化空间。思路是对的，但一些具体步骤没有做到最优
