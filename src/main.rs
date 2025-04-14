#![allow(dead_code)]


#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None,
        }
    }
}

//
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
//
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    // Checks if the linked list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    // Adds a new eleement at the beginning of the list (head)
    pub fn push(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }
    // Removes and returns the elements at the beginnig of the list (head)
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
    // Returns a reference to the element at the beginning of the list (head).
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    // Appends a new element at the end of the list
    pub fn append(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        if self.is_empty() {
            self.head = Some(new_node);
            return;
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node);
                break;
            }
            current = &mut node.next;
        }
    }
    // To list
    pub fn to_list(&mut self, vec: Vec<T>) {
        for e in vec {
            self.append(e);
        }
    }
    // To print
    pub fn to_print(&self)
    where T: std::fmt::Display, {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.next.is_none(){
                print!("None");
                return;
            }
            print!("{} -> ", node.data);
            current = &node.next;
        }
    }
}
//
fn main() -> Result<(), String> {
    let mut list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.is_empty(), true);
    list.push(10);
    list.push(20);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.pop(), Some(20));

    println!("{:?}", list);
    let mut list2: LinkedList<i32> = LinkedList::new();
    list2.to_list(vec![1, 2, 3, 4]);
    println!("{:?}", list2);
    list2.to_print();

    Ok(())
}
