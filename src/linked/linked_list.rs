pub struct LinkedList {
    pub head: Option<Box<Node>>,
}

pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}


impl LinkedList {
   pub fn new() -> Self { 
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Node {
            value: value,
            next: self.head.take()
        };
        
        self.head = Some(Box::new(new_node))
    }
    
    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}" ,node.value);
            current = &node.next;
        }
        println!();

    }

}
