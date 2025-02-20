use std::{iter::once, ops::Deref as _};

use super::algebra::TypeName;

pub fn infer_program<'db>(db: &'db dyn crate::Db, program: ast::Program<'db>) -> ir::Program<'db> {
    let mut converter = Converter { db };
    let expr = converter.convert_annotate_expr(program.expr(db));
    let binding = Binding::Value(vec![ValueDefinition {
        quantifiers: vec![],
        pattern: VariableId::new(db, "program".to_string()),
        expr,
    }]);
    infer_binding(binding);

    //let mut env = Default::default();
    //env = funcs::binary::inject_type_env(db, env, &mut schemes, &mut sub);

    //ir::Program::new(db, expr, inferencer.schemes, inferencer.sub.kind_env)
    todo!()
}

pub enum Binding<'db> {
    Value(Vec<ValueDefinition<'db>>),
}

pub struct ValueDefinition<'db> {
    quantifiers: Vec<TypeName<'db>>,
    pattern: VariableId<'db>,
    expr: ir::LocatedExpression<'db>,
}

pub fn infer_binding(binding: Binding) {
    match binding {
        Binding::Value(vdefs) => {
            let schemes = vdefs.map(|vdef| infer_vdef(vdef, tenv));
            todo!()
        }
    }
}

struct Converter<'db> {
    db: &'db dyn crate::Db,
}

impl<'db> Converter<'db> {
    // Converts an [`ast::Expression`] to an [`ir::Expression`] while annotating with type
    // variables.
    fn convert_annotate_expr(
        &mut self,
        located_expr: &Located<ast::Expression<'db>>,
    ) -> Located<ir::Expression<'db>> {
        located_expr.map(|expr| match expr {
            ast::Expression::Let {
                identifier,
                init,
                body,
            } => ir::Expression::Let {
                identifier: *identifier,
                init: ir::ExpressionRef::new(self.convert_annotate_expr(init)),
                body: ir::ExpressionRef::new(self.convert_annotate_expr(body)),
                scheme_idx: crate::inference::schemes::SchemeIdx::MAX,
            },
            ast::Expression::Integer(i) => ir::Expression::Integer(*i),
            ast::Expression::Float(f) => ir::Expression::Float(*f),
            ast::Expression::Identifier(ident) => ir::Expression::Variable {
                name: *ident,
                typ: Type::Error,
            },
            ast::Expression::Binary { left, op, right } => ir::Expression::Application {
                func: ir::ExpressionRef::new(
                    (
                        ir::Expression::Variable {
                            name: ir::VariableId::new(self.db, binary_op_name(op).to_string()),
                            typ: Type::Error,
                        },
                        located_expr.location(),
                    )
                        .into(),
                ),
                parameters: vec![
                    self.convert_annotate_expr(left),
                    self.convert_annotate_expr(right),
                ],
                typ: Type::Error,
            },
            ast::Expression::Forward { parameter, call } => match &call.value {
                ast::Expression::Call { callee, parameters } => ir::Expression::Application {
                    func: ir::ExpressionRef::new(self.convert_annotate_expr(callee)),
                    parameters: once(parameter.deref())
                        .chain(parameters.iter())
                        .map(|expr| self.convert_annotate_expr(expr))
                        .collect(),
                    typ: Type::Error,
                },
                _ => todo!("record diagnostic, must pipe forward into call expression"),
            },
            ast::Expression::Call { callee, parameters } => ir::Expression::Application {
                func: ir::ExpressionRef::new(self.convert_annotate_expr(callee)),
                parameters: parameters
                    .iter()
                    .map(|expr| self.convert_annotate_expr(expr))
                    .collect(),
                typ: Type::Error,
            },
            ast::Expression::Function { parameters, body } => ir::Expression::Abstraction {
                parameters: parameters.clone(),
                body: Box::new(self.convert_annotate_expr(body)),
                typ: Type::Error,
            },
            ast::Expression::Record { fields } => ir::Expression::Record {
                fields: fields
                    .iter()
                    .map(|(label, field)| (label.clone(), self.convert_annotate_expr(field)))
                    .collect(),
                typ: Type::Error,
            },
            ast::Expression::RecordSelect { record, label } => ir::Expression::RecordSelect {
                record: Box::new(self.convert_annotate_expr(record)),
                label: label.clone(),
                typ: Type::Error,
            },
            ast::Expression::List { .. } => todo!(),
            ast::Expression::RecordConcat { left, right } => ir::Expression::RecordConcat {
                left: Box::new(self.convert_annotate_expr(left)),
                right: Box::new(self.convert_annotate_expr(right)),
                typ: Type::Error,
            },
            ast::Expression::Error => ir::Expression::Error,
        })
    }
}
fn binary_op_name(op: &ast::BinaryOp) -> &'static str {
    match op {
        ast::BinaryOp::Addition => "+",
        ast::BinaryOp::Subtraction => "-",
        ast::BinaryOp::Multiplication => "*",
        ast::BinaryOp::Division => "/",
    }
}
