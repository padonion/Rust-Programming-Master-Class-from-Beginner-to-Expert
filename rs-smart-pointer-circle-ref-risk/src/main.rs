use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    next: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {:?}!", self);
    }
}

fn main() {

    let a = Rc::new(RefCell::new(Node { next: None }));
    println!("- After creating a:");
    println!("a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));

    let b = Rc::new(RefCell::new(Node { next: Some(Rc::downgrade(&a)) }));
    println!("- After creating b:");
    println!("a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));

    let c = Rc::new(RefCell::new(Node { next: Some(Rc::downgrade(&b)) }));
    println!("- After creating c:");
    println!("a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("c strong = {}, weak = {}", Rc::strong_count(&c), Rc::weak_count(&c));

    //  following line to create a cycle
    (*a).borrow_mut().next = Some(Rc::downgrade(&c));
    println!("- After creating cycle:");
    println!("a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("c strong = {}, weak = {}", Rc::strong_count(&c), Rc::weak_count(&c));

    //  following line will create a cycle
    println!("a is {:?}", a);

    // break the cycle (not needed anymore)
    //(*a).borrow_mut().next = None;
}
