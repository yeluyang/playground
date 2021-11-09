/*
 * @lc app=leetcode.cn id=36 lang=rust
 *
 * [36] 有效的数独
 */

// @lc code=start
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut columns = vec![0u16; 9];
        let mut rows = vec![0u16; 9];
        let mut grids = vec![0u16; 9];
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if !board[i][j].is_ascii_digit() {
                    continue;
                }
                let k = 3 * (i / 3) + j / 3;
                let p = 1 << (board[i][j] as u8 - b'0');
                if columns[i] & p != 0 {
                    return false;
                } else {
                    columns[i] += p;
                }
                if rows[j] & p != 0 {
                    return false;
                } else {
                    rows[j] += p;
                }
                if grids[k] & p != 0 {
                    return false;
                } else {
                    grids[k] += p;
                }
            }
        }
        true
    }
}
// @lc code=end
