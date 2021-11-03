/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let end = nums.len();
        quick_select(nums.as_mut_slice(), k, 0, end)
    }
}

fn quick_select(nums: &mut [i32], k: i32, start: usize, end: usize) -> i32 {
    use rand::Rng;
    let i = rand::thread_rng().gen_range(start, end);
    let x = quick_sort_part(nums, i, start, end);
    if x + 1 < k as usize {
        quick_select(nums, k, x + 1, end)
    } else if x + 1 > k as usize {
        quick_select(nums, k, start, x)
    } else {
        nums[x]
    }
}
fn quick_sort_part(nums: &mut [i32], guard: usize, start: usize, end: usize) -> usize {
    nums.swap(guard, end - 1);
    let mut j = start;
    for i in start..end - 1 {
        if nums[i] >= nums[end - 1] {
            nums.swap(i, j);
            j += 1;
        }
    }
    nums.swap(j, end - 1);
    j
}
// @lc code=end

/* [wrong] 2021/11/12:1 没有做出最优解
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut topk = vec![i32::MIN; k as usize];
    for mut n in nums {
        for i in 0..k as usize {
            if n > topk[i] {
                let tmp = topk[i];
                topk[i] = n;
                n = tmp;
            }
        }
    }
    *topk.last().unwrap()
}
*/
