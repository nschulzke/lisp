use crate::env::Env;
use crate::error::RuntimeError;
use crate::object::Object;

fn eval(object: Object, env: &mut Env) -> Result<Object, RuntimeError> {
    match object {
        Object::Integer(i) => Ok(Object::Integer(i)),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::Boolean(b) => Ok(Object::Boolean(b)),
        Object::List(l) => eval_list(l, env),
        Object::Function(_, _) => Err(RuntimeError {
            message: "Cannot eval function".to_string(),
        }),
        Object::PrimitiveFunction(_, _) => Err(RuntimeError {
            message: format!("Cannot eval primitive function {}", object),
        }),
        Object::SpecialForm(_, _) => Err(RuntimeError {
            message: format!("Cannot eval special form {}", object),
        }),
    }
}

fn eval_symbol(s: String, env: &mut Env) -> Result<Object, RuntimeError> {
    match env.get(&s) {
        Some(Object::Symbol(s)) => eval_symbol(s, env),
        Some(o) => Ok(o),
        None => Err(RuntimeError {
            message: format!("Unknown symbol: {}", s),
        }),
    }
}

fn eval_list(mut list: Vec<Object>, env: &mut Env) -> Result<Object, RuntimeError> {
    if list.len() == 0 {
        return Ok(Object::List(vec![]));
    }
    let head = eval(list.remove(0), env)?;
    match head {
        Object::Function(params, body) => eval_function(params, body, list, env),
        Object::PrimitiveFunction(name, 2) => eval_primitive_binary(name, list, env),
        Object::SpecialForm(name, arity) => eval_special_form(name, arity, list, env),
        _ => {
            return Err(RuntimeError {
                message: format!("Expected callable, got {}", head),
            });
        }
    }
}

fn eval_function(params: Vec<String>, body: Vec<Object>, arguments: Vec<Object>, env: &mut Env) -> Result<Object, RuntimeError> {
    let mut new_env = env.extend();
    for (param, argument) in params.iter().zip(arguments.into_iter()) {
        new_env.set(param, argument);
    }
    eval_list(body, &mut new_env)
}

fn eval_primitive_binary(name: String, mut list: Vec<Object>, env: &mut Env) -> Result<Object, RuntimeError> {
    if list.len() != 2 {
        return Err(RuntimeError {
            message: format!("Expected 2 arguments to {}", name),
        });
    }
    let a = eval(list.remove(0), env)?;
    let b = eval(list.remove(0), env)?;
    match name.as_str() {
        "+" => Ok(Object::Integer(a.into_integer()? + b.into_integer()?)),
        "-" => Ok(Object::Integer(a.into_integer()? - b.into_integer()?)),
        "*" => Ok(Object::Integer(a.into_integer()? * b.into_integer()?)),
        "/" => Ok(Object::Integer(a.into_integer()? / b.into_integer()?)),
        "=" => Ok(Object::Boolean(a.into_integer()? == b.into_integer()?)),
        "!=" => Ok(Object::Boolean(a.into_integer()? != b.into_integer()?)),
        "<" => Ok(Object::Boolean(a.into_integer()? < b.into_integer()?)),
        ">" => Ok(Object::Boolean(a.into_integer()? > b.into_integer()?)),
        "<=" => Ok(Object::Boolean(a.into_integer()? <= b.into_integer()?)),
        ">=" => Ok(Object::Boolean(a.into_integer()? >= b.into_integer()?)),
        _ => unreachable!(),
    }
}

fn eval_special_form(name: String, arity: Option<usize>, mut list: Vec<Object>, env: &mut Env) -> Result<Object, RuntimeError> {
    if let Some(arity) = arity {
        if list.len() != arity {
            return Err(RuntimeError {
                message: format!("Expected {} arguments to {}", arity, name),
            });
        }
    }
    match name.as_str() {
        "quote" => Ok(list.remove(0)),
        "progn" => eval_progn(list, env),
        "let" => eval_let(list, env),
        "if" => eval_if(list, env),
        "def" => eval_def(list, env),
        "fn" => eval_fn(list, env),
        _ => Err(RuntimeError {
            message: format!("Unknown special form: {}", name),
        }),
    }
}

fn eval_progn(list: Vec<Object>, env: &mut Env) -> Result<Object, RuntimeError> {
    let mut result = Object::List(vec![]);
    for item in list {
        result = eval(item, env)?;
    }
    Ok(result)
}

fn eval_let(mut list: Vec<Object>, env: &mut Env) -> Result<Object, RuntimeError> {
    let definitions = list.remove(0).into_list()?;
    let body = list.remove(0);
    let mut new_env = env.extend();
    for definition in definitions {
        match definition {
            Object::List(mut list) => {
                let name = list.remove(0);
                let value = eval(list.remove(0), env)?;
                new_env.set(&name.into_symbol()?, value);
            }
            _ => {
                return Err(RuntimeError {
                    message: format!("Expected list, got {}", definition),
                });
            }
        }
    }
    eval(body, &mut new_env)
}

fn eval_if(mut list: Vec<Object>, env: &mut Env) -> Result<Object, RuntimeError> {
    let condition = eval(list.remove(0), env)?;
    let true_branch = list.remove(0);
    let false_branch = list.remove(0);
    if condition.into_boolean()? {
        eval(true_branch, env)
    } else {
        eval(false_branch, env)
    }
}

fn eval_def(mut list: Vec<Object>, env: &mut Env) -> Result<Object, RuntimeError> {
    let name = list.remove(0).into_symbol()?;
    let value = eval(list.remove(0), env)?;
    env.set(&name, value);
    Ok(Object::Boolean(true))
}

fn eval_fn(mut list: Vec<Object>, _env: &mut Env) -> Result<Object, RuntimeError> {
    let mut params = vec![];
    for param in list.remove(0).into_list()? {
        params.push(param.into_symbol()?);
    }
    let body = list.remove(0).into_list()?;
    Ok(Object::Function(params, body))
}

#[cfg(test)]
mod tests {
    use crate::env::Env;
    use crate::interpreter::eval;
    use crate::object::Object;
    use crate::parser::parse;

    #[test]
    fn eval_arithmetic() {
        let input = "(+ 1 2)";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn eval_arithmetic_nested() {
        let input = "(+ 1 (* 2 3))";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(7));
    }

    #[test]
    fn eval_arithmetic_triply_nested() {
        let input = "(+ 1 (* 2 (+ 3 4)))";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(15));
    }

    #[test]
    fn eval_let() {
        let input = "(let ((a 1) (b 2)) (+ a b))";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn eval_lets_nested() {
        let input = "(let ((a 1) (b 2)) (let ((c 3) (d 4)) (+ a (+ b (+ c d)))))";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(10));
    }

    #[test]
    fn eval_if() {
        let input = "(if #true 1 2)";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(1));
    }

    #[test]
    fn eval_ifs_nested() {
        let input = "(if #true (if #false 1 2) 3)";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(2));
    }

    #[test]
    fn eval_def() {
        let input = "(def a 1)";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Boolean(true));
        assert_eq!(env.get("a").unwrap(), Object::Integer(1));
    }

    #[test]
    fn eval_def_use_later() {
        let input = "(progn (def a 1) (+ a 1))";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(2));
    }

    #[test]
    fn eval_def_fn_use_later() {
        let input = "(progn (def a (fn (x) (+ x 1))) (a 1))";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(2));
    }

    #[test]
    fn eval_fn() {
        let input = "(fn (a) (+ a 1))";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Function(vec!["a".into()], vec![Object::Symbol("+".into()), Object::Symbol("a".into()), Object::Integer(1)]));
    }

    #[test]
    fn eval_fn_call() {
        let input = "((fn (a) (+ a 1)) 1)";
        let mut env = Env::base();
        let result = eval(parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(result, Object::Integer(2));
    }
}
