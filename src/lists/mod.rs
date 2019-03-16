// src/lists/mod.rs
use std::cell::RefCell;
use std::rc::Rc;

mod node;

use crate::lists::node::{ Node, NodeOption, ListNodeIterator };

#[derive(PartialEq, Debug)]
pub struct LinkedList {
    head: NodeOption,
    tail: NodeOption,
    pub length: usize
}

impl LinkedList {

    pub fn new_empty() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn new(text: String) -> Self {
        let new_head = Node::new(text);

        LinkedList {
            head: Some(new_head),
            tail: None,
            length: 1
        }
    }

    pub fn append_start(&mut self, text: String) {
        let new_head = Node::new(text);

        match self.head.take() {
            Some(old_head) => {
                new_head.borrow_mut().next = Some(Rc::clone(&old_head));

                match &self.tail {
                    None => {
                        self.tail = Some(Rc::clone(&old_head));
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        self.head = Some(new_head);
        self.length = self.length + 1;
    }

    pub fn append_end(&mut self, text: String) {

        match &self.head {
            Some(head) => {
                let new_tail = Node::new(text);

                match self.tail.take() {
                    Some(old_tail) => {
                        old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));        
                    },
                    _ => {
                        head.borrow_mut().next = Some(Rc::clone(&new_tail));
                    }    
                }

                self.tail = Some(new_tail);
                self.length = self.length + 1;

            },
            _ => {
                self.append_start(text);
            }
        }
    }

    pub fn pop_head(&mut self) -> Option<String> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.head = Some(Rc::clone(&new_head));
                },
                _ => {}
            }
            self.length = self.length - 1;
            old_head.borrow().data.clone()
        })
    }

    pub fn pop_end(&mut self) -> Option<String> {
        self.tail.take().map(|old_tail| {

            let mut iterator = self.iter_node();
            let mut temp = iterator.next();
            

            for _ in 0..self.length - 2 {
                temp = iterator.next();
            }

            match temp {
                Some(node) => {
                    node.borrow_mut().next = None;

                    if self.length > 2 {
                        self.tail = Some(Rc::clone(&node));
                    }
                },
                _ => {}
            }
            
            self.length = self.length - 1;
            old_tail.borrow().data.clone()
        })
    }

    pub fn print_items(&self) {
        for node in self.iter_node() {
            println!("the data is {}", node.borrow().data);
        }
    }

    fn iter_node(&self) -> ListNodeIterator {
        match &self.head {
            Some(head) => {
                ListNodeIterator::new(Some(Rc::clone(head)))
            },
            _ => ListNodeIterator::new(None)
        }
    }

}

mod tests {

    use super::*;
    
    #[test]
    fn test_new_empty_list() {
        let list = LinkedList::new_empty();

        assert_eq!(list, LinkedList {
            head: None,
            tail: None,
            length: 0
        });
    }

    #[test]
    fn test_new_list() {
        let list = LinkedList::new("node_1".to_string());

        assert_eq!(list, LinkedList {
            head: Some(Node::new("node_1".to_string())),
            tail: None,
            length: 1
        });
    }

    #[test]
    fn test_linked_list_append_start() {
        let s = "node_1".to_string();
        let c = "node_2".to_string();

        let tail = Node::new(s.clone());

        let head = Node {
            data: c.clone(),
            next: Some(Rc::clone(&tail))
        };

        let list = LinkedList {
            head: Some(Rc::new(RefCell::new(head))),
            tail: Some(tail),
            length: 2
        };

        
        let mut l_list = LinkedList::new_empty();
        l_list.append_start(s);
        l_list.append_start(c);

        assert_eq!(l_list, list);
    }

     #[test]
    fn test_linked_list_append_end() {
        let s = "node_1".to_string();
        let c = "node_2".to_string();

        let tail = Node::new(c.clone());

        let head = Node {
            data: s.clone(),
            next: Some(Rc::clone(&tail))
        };

        let list = LinkedList {
            head: Some(Rc::new(RefCell::new(head))),
            tail: Some(tail),
            length: 2
        };

        
        let mut l_list = LinkedList::new_empty();
        l_list.append_end(s);
        l_list.append_end(c);

        assert_eq!(l_list, list);
    }

    #[test]
    fn test_pop_head() {
        let mut list = LinkedList::new("node_1".to_string());

        assert_eq!(list.pop_head(), Some("node_1".to_string()));
    }

    #[test]
    fn test_pop_end() {
        let mut list = LinkedList::new("node_1".to_string());
        list.append_end("node_2".to_string());
        list.append_end("node_5".to_string());
        list.append_end("node_3".to_string());
        list.append_end("node_4".to_string());
        assert_eq!(list.pop_end(), Some("node_4".to_string()));
    }
}