const prompt = require("prompt-sync")({sigint: true});
const LinkedList = require("./linkedList");
const { performance } = require("perf_hooks");

let nodeCount = 0;
let linkedList = null;

const printList = (list) => {
  let cur = list.head;
  let str = "";

  while (cur != null) {
    str += cur.data + " ";
    if (cur.next != null) {
      str += "-> ";
    }
    cur = cur.next;
  }

  return str;
}

while(nodeCount < 1 || Number.isNaN(nodeCount)) {
  nodeCount = parseInt(prompt("How many nodes in the linked list? "));
  if (nodeCount > 0) {
    console.log("Generating linked list with " + nodeCount + " node(s)");
    linkedList = new LinkedList(nodeCount);
    console.log(printList(linkedList));
  } else {
    console.log("Not a valid number");
  }
}

// Recursive version for fun but since Rust doesn't like recursion
// going with iterative approach for comparison
// const reverse = (head) => {
//   if (!head || !head.next) {
//     return head;
//   }
//   let newHead = reverse(head.next);

//   head.next.next = head;
//   head.next = null;

//   return newHead;
// }

const reverse = (head) => {
  let cur = head;
  let prev = null;
  let next;

  while(cur != null) {
    next = cur.next;
    cur.next = prev;
    prev = cur;
    cur = next;
  }

  return prev;
}

console.log("Reversing the linked list");
const start = performance.now();
linkedList.head = reverse(linkedList.head);

const end = performance.now();

console.log(printList(linkedList));

console.log("Execution time: " + ((end - start) * 1000000) + "ns");

process.exit();