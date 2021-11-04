/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = binary_search(&nums, target, true);
        let right = binary_search(&nums, target, false) - 1;
        if left < right && right < nums.len() && nums[left] == target && nums[right] == target {
            vec![left as i32, right as i32]
        } else {
            vec![-1; 2]
        }
    }
}
fn binary_search(nums: &Vec<i32>, target: i32, left: bool) -> usize {
    let mut start = 0;
    let mut end = nums.len();
    while start < end {
        let mid = start + (end - start) / 2;
        if target < nums[mid] || (left && target == nums[mid]) {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    end
}
// @lc code=end

/* [wrong] 2021/11/03-1 没有做出最优解，且答案无法理解，无法证明答案的正确性，抄都抄不对
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ret = vec![-1; 2];
    let mut start = 0;
    let mut end = nums.len();
    while start < end {
        let mid = start + (end - start) / 2;
        if nums[mid] == target {
            let mut i = mid;
            while i > 0 && nums[i - 1] == target {
                i -= 1;
            }
            ret[0] = i as i32;
            i = mid;
            while i < nums.len() - 1 && nums[i + 1] == target {
                i += 1;
            }
            ret[1] = i as i32;
            break;
        } else if nums[mid] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    ret
}
*/
