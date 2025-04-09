#![allow(dead_code)]

#[derive(Debug, PartialEq)]
struct Node {
    value: i32,
    next: Link,
}

impl Node {
    fn new(data: i32) -> Self {
        Self {
            value: data,
            next: None,
        }
    }
}
type Link = Option<Box<Node>>;

#[derive(Debug, PartialEq)]
struct LinkeList {
    head: Link,
}

impl LinkeList {
    fn new() -> Self {
        Self { head: None }
    }
    // Push an element to the front of the list (O(1))
    fn push_front(&mut self, data: i32) {
        let new_head = Box::new(Node {
            value: data,
            next: self.head.take(),
        });
        self.head = Some(new_head);
    }
    // Pop an element from the front of the list (O(1))
    fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
    //
    fn peek(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.value)
    }
    //
    fn push_back(&mut self, value: i32) {
        let new_node = Box::new(Node::new(value));
        //
        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(new_node);
            }
        }
    }
    //
    fn pop_back(&mut self) -> Option<i32> {
        match self.head.as_mut() {
            None => None,
            Some(node) if node.next.is_none() => {
                return self.pop_front();
            }
            Some(_) => {
                let mut current = self.head.as_mut().unwrap();

                while current.next.as_ref().unwrap().next.is_none() {
                    current = current.next.as_mut().unwrap();
                }

                let last_node = current.next.take().unwrap();
                Some(last_node.value)
            }
        }
    }
    //
    fn length(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
    //
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    //
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() -> Result<(), String> {
    let mut lst = LinkeList::new();
    lst.push_back(10);
    lst.push_back(20);
    lst.push_back(30);

    lst.print();
    Ok(())
}
