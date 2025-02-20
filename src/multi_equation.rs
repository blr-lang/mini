use std::{
    cell::RefCell,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::{
    algebra::Term,
    union_find::{self, UnionFind},
};

pub struct TName(pub String);

pub type Variable<'db> = union_find::Point<Descriptor<'db>>;

#[derive(Debug)]
pub struct Descriptor<'db> {
    structure: Option<Term<'db, Variable<'db>>>,
    rank: usize,
    mark: Mark,
    kind: Kind,
    name: Option<TypeName<'db>>,
    var: Option<Variable<'db>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Rigid,
    Flexible,
    Constant,
}

struct Pool<'db> {
    number: usize,
    inhabitants: RefCell<Vec<Variable<'db>>>,
}

impl<'db> Pool<'db> {
    fn init() -> Self {
        Self {
            number: 0,
            inhabitants: Default::default(),
        }
    }
    fn new(pool: &Pool) -> Self {
        Self {
            number: pool.number + 1,
            inhabitants: Default::default(),
        }
    }
    /// Adds v to the pool without modifing v's rank.
    fn register(&self, v: &Variable<'db>) {
        self.inhabitants.borrow_mut().push(v.clone());
    }
    /// Adds v to the pool setting v's rank to the pool's number.
    fn introduce(&self, v: &Variable<'db>) {
        UnionFind::find_map(v, |desc| desc.rank = self.number);
        self.register(v)
    }
    fn instance(&self, v: Variable<'db>) -> Variable<'db> {
        let m = Mark::fresh();
        let new_v = self.copy(m, &v);
        Self::restore(m, &v);
        new_v
    }
    fn copy(&self, m: Mark, v: &Variable<'db>) -> Variable<'db> {
        UnionFind::find_map(v, |desc| {
            if desc.mark == m {
                desc.var.as_ref().expect("should have variable").clone()
            } else if desc.rank != 0 || desc.kind == Kind::Constant {
                v.clone()
            } else {
                let new_desc = Descriptor {
                    structure: desc
                        .structure
                        .as_ref()
                        .map(|term| term.map(|v| self.copy(m, v))),
                    rank: self.number,
                    mark: Mark::none(),
                    kind: Kind::Flexible,
                    name: match desc.kind {
                        Kind::Rigid => None,
                        _ => desc.name,
                    },
                    var: None,
                };
                let new_v = UnionFind::fresh(new_desc);
                self.register(&new_v);
                desc.mark = m;
                desc.var = Some(new_v.clone());

                new_v
            }
        })
    }
    fn restore(m: Mark, v: &Variable<'db>) {
        UnionFind::find_map(v, |desc| {
            if desc.mark == m {
                desc.mark = Mark::none();
                desc.rank = 0;
            }
            if let Some(term) = desc.structure.as_ref() {
                term.iter(|v| Self::restore(m, v))
            }
        });
    }
}

fn is_rigid(v: &Variable) -> bool {
    UnionFind::find_map(v, |desc| desc.kind == Kind::Rigid)
}
fn is_flexible(v: &Variable) -> bool {
    UnionFind::find_map(v, |desc| desc.kind == Kind::Flexible)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mark(usize);

impl Mark {
    fn fresh() -> Mark {
        static MARK: AtomicUsize = AtomicUsize::new(1);
        Mark(MARK.fetch_add(1, Ordering::SeqCst))
    }
    fn none() -> Mark {
        Self(0)
    }
}

pub fn variable<'db>(
    kind: Kind,
    name: Option<TypeName<'db>>,
    structure: Option<Term<'db, Variable<'db>>>,
) -> Variable<'db> {
    UnionFind::fresh(Descriptor {
        structure,
        rank: 0,
        mark: Mark::none(),
        kind,
        name,
        var: None,
    })
}
