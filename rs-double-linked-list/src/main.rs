use std::cell::RefCell;
use std::rc::Rc;

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

struct List<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

pub struct Node<T> {
    element: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

impl<T: std::fmt::Display> Node<T> {
    fn new(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            element: element,
            prev: None,
            next: None,
        }))
    }
}

impl<T: std::fmt::Display> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, element: T) {
        let new_head = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, element: T) {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().element
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().element
        })
    }

    fn print(&self) {
        if self.head.is_none() {
            println!("[]");
        } else {
            let mut transversal = self.head.clone();
            print!("[");
            while transversal.is_some() {
                print!("{} ", transversal.as_ref().unwrap().borrow().element);
                transversal = transversal.unwrap().borrow().next.clone();
            }
            println!("]");
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.print();
    list.pop_front();
    list.pop_back();
    list.print();
}
