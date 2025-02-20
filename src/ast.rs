use crate::{
    algebra::{self, TypeName},
    multi_equation,
};

struct Program {
    bindings: Vec<Binding>,
}

enum Binding {
    Value(Vec<ValueDefinition>),
    RecValue(Vec<ValueDefinition>),
    TypeDec(Vec<TypeDeclaration>),
}

enum Expression {
    Var(Name),
    Lambda(Pattern, Box<Expression>),
    App(Box<Expression>, Box<Expression>),
    Binding(Binding, Box<Expression>),
    PrimApp(Primitive, Vec<Expression>),
    Forall(Vec<TName>, Box<Expression>),
    Exists(Vec<TName>, Box<Expression>),
    TypeConstraint(Box<Expression>, Type),
    DCon(DName, Vec<Expression>),
    Match(Box<Expression>, Vec<Clause>),
    RecordEmpty,
    RecordAccess(Box<Expression>, LName),
    RecordExtend(Vec<RecordBinding>, Box<Expression>),
    RecordUpdate(Box<Expression>, LName, Box<Expression>),
    AssertFalse,
}

struct Name(String);
type TName = multi_equation::TName;
struct DName(String);
type LName = algebra::LName;

enum Primitive {
    IntegerConstant(i32),
    CharConstant(char),
    Unit,
}

struct Clause(Pattern, Expression);
struct RecordBinding(LName, Expression);

struct TypeDeclaration(Kind, TName, TypeDefinition);

struct TypeDefinition(Vec<(DName, Vec<TName>, Type)>);

struct ValueDefinition {
    quantifiers: Vec<TypeName>,
    pattern: Pattern,
    expr: Expression,
}

enum Pattern {
    Var(Name),
    Wildcard,
    Alias(Name, Box<Pattern>),
    TypeConstraint(Box<Pattern>, Type),
    Primitive(Primitive),
    Data(DName, Vec<Pattern>),
    And(Vec<Pattern>),
    Or(Vec<Pattern>),
}

enum Kind {
    Star,
    Times(Box<Kind>, Box<Kind>),
    Arrow(Box<Kind>, Box<Kind>),
    EmptyRow,
}
enum Type {
    Var(TName),
    App(Box<Type>, Vec<Type>),
    RowCons(Vec<(LName, Type)>, Box<Type>),
    RowUniform(Box<Type>),
}
