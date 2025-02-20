//mod algebra;
//mod ast;
//mod constraint;
//mod kind_inferencer;
//mod multi_equation;
//mod set_equations;
//mod typing_environment;
//mod union_find;

pub async fn infer(src: &str) -> Result<String, ()> {
    Ok(format!("Inferred: {src}"))
}
