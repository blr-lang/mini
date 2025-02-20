use std::collections::BTreeMap;

use super::{
    algebra::{ARTerm, TypeName},
    multi_equation::{self, variable, Kind},
};

pub type Variable<'db> = multi_equation::Variable<'db>;
pub type CRTerm<'db> = ARTerm<'db, Variable<'db>>;
pub type TConstraint<'db> = TypeConstraint<'db, CRTerm<'db>, Variable<'db>>;
pub type TScheme<'db> = Scheme<'db, CRTerm<'db>, Variable<'db>>;

pub enum TypeConstraint<'db, C, V> {
    True,
    Dump,
    Equation(C, C),
    Conjunction(Vec<TypeConstraint<'db, C, V>>),
    Let(Vec<Scheme<'db, C, V>>, Box<TypeConstraint<'db, C, V>>),
    Instance(SchemeName<'db>, C),
    Disjunction(Vec<TypeConstraint<'db, C, V>>),
}

#[salsa::interned]
pub struct SchemeName<'db> {
    #[return_ref]
    pub name: String,
}
pub struct Scheme<'db, C, V> {
    rigid: Vec<V>,
    flexible: Vec<V>,
    constraint: TypeConstraint<'db, C, V>,
    header: BTreeMap<String, C>,
}

pub fn instance<'db>(x: SchemeName<'db>, term: CRTerm<'db>) -> TConstraint<'db> {
    TypeConstraint::Instance(x, term)
}
pub fn equality<'db>(t1: CRTerm<'db>, t2: CRTerm<'db>) -> TConstraint<'db> {
    TypeConstraint::Equation(t1, t2)
}
pub fn conjunction<'db>(c1: TConstraint<'db>, c2: TConstraint<'db>) -> TConstraint<'db> {
    match (c1, c2) {
        (TypeConstraint::True, c) | (c, TypeConstraint::True) => c,
        (c, TypeConstraint::Conjunction(mut cs)) => {
            cs.push(c);
            TypeConstraint::Conjunction(cs)
        }
        (c1, c2) => TypeConstraint::Conjunction(vec![c1, c2]),
    }
}
pub fn conjunctions<'db>(cs: impl IntoIterator<Item = TConstraint<'db>>) -> TConstraint<'db> {
    TypeConstraint::Conjunction(cs.into_iter().collect())
}
pub fn ex<'db>(quantifiers: Vec<Variable<'db>>, constraint: TConstraint<'db>) -> TConstraint<'db> {
    TypeConstraint::Let(
        vec![Scheme {
            rigid: Default::default(),
            flexible: quantifiers,
            constraint,
            header: Default::default(),
        }],
        TypeConstraint::True.into(),
    )
}
pub fn fl<'db>(quantifiers: Vec<Variable<'db>>, constraint: TConstraint<'db>) -> TConstraint<'db> {
    TypeConstraint::Let(
        vec![Scheme {
            rigid: quantifiers,
            flexible: Default::default(),
            constraint,
            header: Default::default(),
        }],
        TypeConstraint::True.into(),
    )
}

pub fn exists<'db>(f: impl FnOnce(CRTerm<'db>) -> TConstraint<'db>) -> TConstraint<'db> {
    let v = multi_equation::variable(multi_equation::Kind::Flexible, None, None);
    let c = f(ARTerm::Variable(v.clone()));
    ex(vec![v], c)
}

pub fn forall_list<'db>(
    l: impl IntoIterator<Item = TypeName<'db>>,
    f: impl FnOnce(Vec<(TypeName<'db>, CRTerm<'db>)>) -> TConstraint<'db>,
) -> TConstraint<'db> {
    let (l, m) = l
        .into_iter()
        .map(|x| {
            let v = variable(Kind::Rigid, Some(x), None);
            (v.clone(), (x, CRTerm::Variable(v)))
        })
        .unzip();
    fl(l, f(m))
}
