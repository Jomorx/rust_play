

use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

pub struct Node<T> {
    value:T,
    next:Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value:T) -> Self<>{
        Self {
            value,
            next:None
        }
    }
    pub fn push(&mut self, value:T) {
        match self.next.as_mut() {
            None => {
                self.next = Some(
                    Box::new(Node {
                        value,
                        next: None
                    })
                )
            }
            Some(nxt) => {
                nxt.as_mut().push(value)
            }
        }
    }
    pub fn pop(&mut self) ->Option<T>{
        match self.next.as_mut() {
            None => {None}
            Some(nxt) => {
                match nxt.next.as_mut() {
                    None => {
                        self.next.take().map(|v|(*v).value)
                    }
                    Some(_) => {
                        nxt.as_mut().pop()
                    }
                }
            }
        }
    }
}

impl<T> Index<u32> for Node<T>{
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        let mut p = self;
        for _ in 0..index {
            match p.next.as_ref() {
                None => {
                    panic!("数组越界")
                }
                Some(nxt) => {
                    p = &(*nxt)
                }
            }

        }
        &p.value
    }
}
impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut p = self;
        let mut is_first = true;
        while let Some(next) = &p.next {
            if !is_first {
                write!(f, ",")?;
            }
            write!(f, "{}", p.value)?;
            is_first = false;
            p = next;
        }
        write!(f, ",{}]", p.value)
    }
}

impl<T> IndexMut<u32> for Node<T> {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        let mut p = self;
        for _ in 0..index {
            match p.next.as_mut() {
                None => {
                    panic!("数组越界")
                }
                Some(nxt) => {
                    p = &mut (*nxt)
                }
            }

        }
        &mut p.value
    }
}