use std::marker::PhantomData;

use super::{
    algebra::{Kind, Label, TypeName},
    union_find,
};

trait Env<'db, T> {
    fn get(name: TypeName<'db>) -> T;
    fn add(name: TypeName<'db>, t: T);
}

pub struct KindInferencer<T> {
    marker: PhantomData<T>,
}

enum Typ<'db> {
    Var(TypeName<'db>),
    App(Box<Typ<'db>>, Vec<Typ<'db>>),
    RowField(Label<'db>, Box<Typ<'db>>),
    RowConcat(Box<Typ<'db>>, Box<Typ<'db>>),
    RowUniform(Box<Typ<'db>>),
}

impl<T> KindInferencer<T> {
    pub fn fresh_kind() -> T {
        todo!()
    }
    pub fn infer<'db>(env: impl Env<'db, T>, typ: Typ<'db>) -> T {
        todo!()
    }
    pub fn intern_kind<'db>(env: impl Env<'db, T>, kind: Kind) -> T {
        todo!()
    }
    pub fn check<'db>(env: impl Env<'db, T>, typ: Typ<'db>, kind: Kind) -> T {
        todo!()
    }
}

type Variable<'db> = union_find::Point<Descriptor<'db>>;

enum Term<'db> {
    App(Variable<'db>, Variable<'db>),
    Row(),
}

struct Descriptor<'db> {
    structure: Option<Term<'db>>,
    name: TypeName<'db>,
    constant: bool,
}
