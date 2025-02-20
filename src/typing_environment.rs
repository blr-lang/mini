use super::{algebra::DataTypeName, multi_equation::Variable};

type DataType<'db> = Vec<(DataTypeName<'db>, Variable<'db>)>;

struct TypeInfo {}
