class ListNode {
  constructor(data) {
    this.data = data;
    this.next = null;
  }
}

module.exports = class LinkedList {
  constructor(nodeCount) {
    this.head = new ListNode(1);

    let curr = this.head;

    for (let i = 2; i < nodeCount + 1; i++) {
      const newNode = new ListNode(i);
      curr.next = newNode;
      curr = newNode;
    }
  }
}
