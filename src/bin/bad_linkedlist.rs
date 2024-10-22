fn main() {
    let mut list = LinkedList::new();
    list.push("Test String 1".to_string());
    list.push("Test String 2".to_string());
    list.push("Test String 3".to_string());
    list.push("Test String 4".to_string());
    parse_pop(list.pop());
    parse_pop(list.pop());
    parse_pop(list.pop());
    parse_pop(list.pop());
    parse_pop(list.pop());
    parse_pop(list.pop());
    parse_pop(list.pop());
    parse_pop(list.pop());
    parse_pop(list.pop());
}

fn parse_pop(node: Option<Box<Node>>) {
    match node {
        Some(node) => match node.value {
            Some(value) => println!("Node had value \"{}\"", value),
            None => println!("Node had no value"),
        },
        None => println!("Empty list"),
    }
}

struct LinkedList {
    root: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { root: None }
    }

    fn push(&mut self, value: String) {
        let mut node = &mut self.root;
        while let Some(unwrapped) = node {
            if unwrapped.next.is_none() {
                unwrapped.set_next(value);
                return ();
            } else {
                node = &mut unwrapped.next;
            }
        }
        self.root = Some(Box::new(Node::new(Some(value), None)));
    }

    fn pop(&mut self) -> Option<Box<Node>> {
        let mut current_node = &mut self.root;
        while current_node.is_some() {
            if current_node.as_ref().unwrap().next.is_none() {
                return current_node.take();
            } else {
                current_node = &mut current_node.as_mut().unwrap().next;
            }
        }
        return None;
    }

    // fn get(&self, index: usize) -> Option<&String> {
    //     let mut current_index = 0;
    //     let mut returned_node = &self.root;
    //     while current_index < index {}
    //     return returned_node;
    // }
}

struct Node {
    next: Option<Box<Node>>,
    value: Option<String>,
}

impl Node {
    fn new(value: Option<String>, next: Option<Box<Node>>) -> Node {
        Node { next, value }
    }

    fn set_next(&mut self, value: String) {
        self.next = Some(Box::new(Node::new(Some(value), None)))
    }
}
