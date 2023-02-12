use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use super::{
    cons::{Cons, ConsIterator},
    Sexpr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub head: Option<Box<Cons>>,
}

impl List {
    pub const NIL: List = List { head: None };

    pub fn car(&self) -> Result<Sexpr, String> {
        self.head
            .as_ref()
            .map(|b| b.car.clone())
            .ok_or_else(|| String::from("Attempted to apply car on nil"))
    }

    #[must_use]
    pub fn cdr(&self) -> List {
        List {
            head: self
                .head
                .as_ref()
                .and_then(|b| b.cdr.as_ref().cloned()),
        }
    }

    #[must_use]
    pub fn cons(&self, val: Sexpr) -> List {
        List {
            head: Some(Box::new(Cons {
                car: val,
                cdr: self.head.clone(),
            })),
        }
    }
}

impl Display for List {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(head) = &self.head {
            write!(formatter, "({})", head.as_ref())
        } else {
            write!(formatter, "NIL")
        }
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = Sexpr;

    type IntoIter = ConsIterator;

    fn into_iter(self) -> Self::IntoIter {
        ConsIterator(self.head.clone())
    }
}
