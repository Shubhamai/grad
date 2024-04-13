use std::collections::HashMap;

use crate::parser::{Error, Expr, Spanned, Value};

pub fn eval_expr<'src>(
    expr: &Spanned<Expr<'src>>,
    // funcs: &HashMap<&'src str, Func<'src>>,
    funcs: &mut Vec<(&'src str, &'src [&'src str], &'src Expr<'src>)>,

    stack: &mut Vec<(&'src str, Value)>,
) -> Result<Value, Error> {
    Ok(match &expr.0 {
        Expr::Error => unreachable!(), // Error expressions only get created by parser errors, so cannot exist in a valid AST
        Expr::Value(val) => val.clone(),
       
        Expr::Local(name) => stack
            .iter()
            .rev()
            .find(|(l, _)| l == name)
            .map(|(_, v)| v.clone())
            // .or_else(|| Some(Value::Func(name)).filter(|_| funcs.contains_key(name)))
            .ok_or_else(|| Error {
                span: expr.1,
                msg: format!("No such variable '{}' in scope", name),
            })?,
        Expr::Let(local, val, body) => {
            let val = eval_expr(val, funcs, stack)?;
            stack.push((local, val));
            let res = eval_expr(body, funcs, stack)?;
            stack.pop();
            res
        }
        Expr::Then(a, b) => {
            eval_expr(a, funcs, stack)?;
            eval_expr(b, funcs, stack)?
        }
       
        Expr::Print(a) => {
            let val = eval_expr(a, funcs, stack)?;
            println!("{}", val);
            val
        }
    })
}
