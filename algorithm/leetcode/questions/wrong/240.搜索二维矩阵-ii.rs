/*
 * @lc app=leetcode.cn id=240 lang=rust
 *
 * [240] 搜索二维矩阵 II
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut col_margin = matrix[0].len();
        for row in matrix {
            while col_margin > 0 {
                if row[col_margin - 1] > target {
                    col_margin -= 1;
                } else if target > row[col_margin - 1] {
                    break;
                } else {
                    return true;
                }
            }
            if col_margin == 0 {
                return false;
            }
        }
        false
    }
}
// @lc code=end
/* [wrong] 2021/11/04-1 没有做出最优解，一些重要性质没有分析出来
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut col_margin = matrix[0].len();
    for row in matrix {
        if row[0] <= target {
            let mut start = 0;
            let mut end = col_margin;
            while start < end {
                let mid = start + (end - start) / 2;
                if row[mid] == target {
                    return true;
                } else if row[mid] < target {
                    start = mid + 1;
                } else {
                    end = mid;
                    col_margin = mid;
                }
            }
        } else {
            break;
        }
    }
    false
}
*/
