/*
 * @lc app=leetcode.cn id=237 lang=c
 *
 * [237] 删除链表中的节点
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
void deleteNode(struct ListNode *node) {}
// @lc code=end

/* [wrong] 2021/10/09-1 没有做出最优解
void deleteNode(struct ListNode *node) {
  struct ListNode *last = node;
  while (node->next != NULL) {
    last = node;
    node->val = node->next->val;
    node = node->next;
  }
  last->next = NULL;
}
*/
