# Rust Linked list
Simple linked list in Rust.

# Structure

![image](https://github.com/Vportacio/rust-linked-list/assets/55148660/6deac27b-76a7-4d82-9b72-900aec28e546)

The linked list consists of two main types:

### struct LinkedList
`pub struct LinkedList {
    pub head: Option<Box<Node>>,
}`

The LinkedList structure represents the linked list and contains a head field that holds the reference to the first node in the list.

### struct Node

`pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}`
The Node structure represents the individual nodes of the list. Each node contains an integer value (value) and a next field that points to the next node in the list.

### Functionalities

#### new()
`pub fn new() -> Self { 
    LinkedList { head: None }
}`

Initializes a new empty linked list.

#### push(value: i32)
`pub fn push(&mut self, value: i32) {
        let new_node = Node {
            value: value,
            next: self.head.take()
        };
        
        self.head = Some(Box::new(new_node))
    }`

Adds a new node with the specified value to the beginning of the list.

#### print()
`pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}" ,node.value);
            current = &node.next;
        }
        println!();

    }`

Prints the values stored in the linked list.

##### Example Usage:
`fn main() {
    let mut list = LinkedList::new();

    list.push(10);
    list.push(20);
    list.push(30);

    list.print();
}`

This is a simple example of creating a list, adding values to it, and printing its elements.

