use std::ops::Deref;

//-- Cons List (similar to Linked List)
/*
    "to cons x onto y": construct a new container instance by putting he element x at the start
                        of this new container, followed by the container y.
     item in a cons list: - the value of the current item
                          - the next item
 */

struct LinkedList<T> {
    value: T,
    /*
        - The Box<T> will point to the next List value (on the heap rather than inside the Cons
        variant.
        - The size for a Box<T> is fixed.
     */
    next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    pub fn values(&self) -> Vec<&T> {
        let mut v = Vec::new();
        v.push(&self.value);

        match &self.next {
            Some(ll) => { v.append(&mut ll.values()) },
            None => {}
        }
        return v;
    }
}

pub fn run_linked_list_ex() {
    let ll1 = LinkedList {
        value: 1,
        next: Some(Box::new(LinkedList {
            value: 2,
            next: Some(Box::new(LinkedList {
                value:3,
                next: None,
            })),
        })),
    };
    let ll2 = LinkedList {
        value: 'a',
        next: Some(Box::new(LinkedList {
            value: 'b',
            next: Some(Box::new(LinkedList {
                value:'c',
                next: None,
            })),
        })),
    };
    println!("ll1 = {:?}", ll1.values());
    println!("ll2 = {:?}", ll2.values());
}

pub fn run_my_box_ex() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    /*
        W/out the Deref trait, compiler can only dereference & references.
        The deref method enables the compiler to take a value of any type that implements Deref
        and call teh deref method to get a & reference that it knows how to dereference.

        On the next line, *y is the same as *(y.deref())
     */
    assert_eq!(5, *y);
}

// The Box<T> type is ultimately defined as a tuple struct with one element
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // defines an associated type for the Deref trait to use
    type Target = T;

    // borrows self and returns and
    // returns a reference to the value we want to access with the * operator
    fn deref(&self) -> &T {
        &self.0
    }
}
