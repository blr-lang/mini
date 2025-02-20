pub struct TName(pub String);
pub struct LName(pub String);

#[derive(Debug)]
pub enum Term<T> {
    RowUniform(T),
    RowConcat(T, T),
    RowField(Label, T),
    App(T, T),
    Var(T),
}

impl<T> Term<T> {
    pub fn iter(&self, f: impl Fn(&T)) {
        match self {
            Term::RowUniform(content) => f(content),
            Term::RowConcat(left, right) => {
                f(left);
                f(right)
            }
            Term::RowField(_, field) => f(field),
            Term::App(l, r) => {
                f(l);
                f(r)
            }
            Term::Var(v) => f(v),
        }
    }
    pub fn map<U>(&self, f: impl (Fn(&T) -> U)) -> Term<U> {
        match self {
            Term::RowUniform(content) => Term::RowUniform(f(content)),
            Term::RowConcat(left, right) => Term::RowConcat(f(left), f(right)),
            Term::RowField(label, field) => Term::RowField(*label, f(field)),
            Term::App(l, r) => Term::App(f(l), f(r)),
            Term::Var(v) => Term::Var(f(v)),
        }
    }
}

/// Abstract recursive term
#[derive(Debug)]
pub enum ARTerm<T> {
    Variable(T),
    Term(Box<Term<ARTerm<T>>>),
}

pub fn uniform<T>(v: ARTerm<T>) -> ARTerm<T> {
    ARTerm::Term(Term::RowUniform(v).into())
}
pub fn rowfield<T>(label: Label, t: ARTerm<T>) -> ARTerm<T> {
    ARTerm::Term(Term::RowField(label, t).into())
}
pub fn rowconcat<T>(left: ARTerm<T>, right: ARTerm<T>) -> ARTerm<T> {
    ARTerm::Term(Term::RowConcat(left, right).into())
}
pub fn n_rowconcat<T>(
    typed_labels: impl IntoIterator<Item = (Label, ARTerm<T>)>,
    right: ARTerm<T>,
) -> ARTerm<T> {
    typed_labels
        .into_iter()
        .map(|(label, term)| rowfield(label, term))
        .fold(right, |acu, term| rowconcat(acu, term))
}

// Mini Algebra

pub enum Kind {
    Star,
    Times(Box<Kind>, Box<Kind>),
    Arrow(Box<Kind>, Box<Kind>),
    EmptyRow,
}
enum Type {
    Var(TypeName),
    App(Box<Type>, Vec<Type>),
    RowUniform(Box<Type>),
    //TODO RowConcat instead?
    RowCons(Vec<(Label, Type)>, Box<Type>),
}

enum Associativity {
    Left,
    Right,
    None,
    EnclosedBy(String, String),
}
fn builtin_env() -> Vec<(
    TypeName,
    bool,
    Associativity,
    i32,
    Kind,
    Vec<(DataTypeName, Vec<TypeName>, Type)>,
)> {
    let arrow_type = |t1, t2| {
        Type::App(
            Type::Var(TypeName::new("->".to_string())).into(),
            vec![t1, t2],
        )
    };
    let tuple_type2 = |t1, t2| {
        Type::App(
            Type::Var(TypeName::new("*".to_string())).into(),
            vec![t1, t2],
        )
    };
    let gen_tvar = |v: &str| Type::Var(TypeName::new(v.to_string()).into());
    vec![
        (
            TypeName::new("pre".to_string()),
            false,
            Associativity::None,
            -1,
            Kind::Arrow(Kind::Star.into(), Kind::Star.into()),
            vec![],
        ),
        (
            TypeName::new("abs".to_string()),
            false,
            Associativity::None,
            -1,
            Kind::Star,
            vec![],
        ),
        (
            TypeName::new("pi".to_string()),
            false,
            Associativity::EnclosedBy("{".to_string(), "}".to_string()),
            -1,
            Kind::Arrow(Kind::EmptyRow.into(), Kind::Star.into()),
            vec![],
        ),
        (
            TypeName::new("->".to_string()),
            true,
            Associativity::Right,
            0,
            Kind::Arrow(
                Kind::Star.into(),
                Kind::Arrow(Kind::Star.into(), Kind::Star.into()).into(),
            ),
            vec![],
        ),
        (
            TypeName::new("*".to_string()),
            true,
            Associativity::None,
            1,
            Kind::Arrow(
                Kind::Star.into(),
                Kind::Arrow(Kind::Star.into(), Kind::Star.into()).into(),
            ),
            vec![(
                DataTypeName::new("_tuple".to_string()),
                vec![
                    TypeName::new("a".to_string()),
                    TypeName::new("b".to_string()),
                ],
                arrow_type(
                    gen_tvar("a"),
                    arrow_type(gen_tvar("b"), tuple_type2(gen_tvar("a"), gen_tvar("b"))),
                ),
            )],
        ),
        (
            TypeName::new("int".to_string()),
            false,
            Associativity::None,
            2,
            Kind::Star,
            vec![],
        ),
        (
            TypeName::new("char".to_string()),
            false,
            Associativity::None,
            2,
            Kind::Star,
            vec![],
        ),
        (
            TypeName::new("unit".to_string()),
            false,
            Associativity::None,
            3,
            Kind::Star,
            vec![(
                DataTypeName::new("unit".to_string()),
                vec![],
                gen_tvar("unit"),
            )],
        ),
    ]
}

pub trait Environment<T> {
    fn symbol(&self, name: TypeName) -> ARTerm<T>;
}

pub fn mkunit<T>(env: &impl Environment<T>) -> ARTerm<T> {
    env.symbol(TypeName::new("unit".to_string()))
}
pub fn pre<T>(env: &impl Environment<T>, term: ARTerm<T>) -> ARTerm<T> {
    let v = env.symbol(TypeName::new("pre".to_string()));
    ARTerm::Term(Term::App(v, term).into())
}
pub fn abs<T>(env: &impl Environment<T>) -> ARTerm<T> {
    env.symbol(TypeName::new("abs".to_string()))
}
pub fn record_constructor<T>(env: &impl Environment<T>, term: ARTerm<T>) -> ARTerm<T> {
    let v = env.symbol(TypeName::new("pi".to_string()));
    ARTerm::Term(Term::App(v, term).into())
}

pub fn arrow<T>(env: &impl Environment<T>, t: ARTerm<T>, u: ARTerm<T>) -> ARTerm<T> {
    let v = env.symbol(TypeName::new("->".to_string()));
    ARTerm::Term(Term::App(ARTerm::Term(Term::App(v, t).into()), u).into())
}

pub fn n_arrow<T>(
    env: &impl Environment<T>,
    ts: impl IntoIterator<Item = ARTerm<T>>,
    u: ARTerm<T>,
) -> ARTerm<T> {
    ts.into_iter().fold(u, |acu, x| arrow(env, acu, x))
}
