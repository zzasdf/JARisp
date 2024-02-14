use crate::expression::{RispEnv, RispExp};
use crate::eval::eval;
use std::collections::HashMap;
use crate::operation;
use std::rc::Rc;

fn begin(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    for (i, item) in tree.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let tem = eval(item, global_env, local_env);
        match &tem {
            RispExp::Number(k) => {
                println!("{}", *k)
            }
            RispExp::Symbol(k) => {
                println!("{}", *k)
            }
            _ => {}
        };
    }
    return RispExp::Nothing();
}

fn list(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let mut result: Vec<RispExp> = Vec::with_capacity(tree.len()-1);
    for (i, item) in tree.iter().enumerate() {
        if i == 0 {
            continue;
        }
        result.push(eval(item, global_env, local_env));
    }
    return RispExp::List(result);
}

fn rif(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    assert_eq!(tree.len(), 4);
    let flag = eval(&tree[1], global_env, local_env);
    match flag {
        RispExp::Bool(bl) => {
            if bl{
                return eval(&tree[2], global_env, local_env)
            }
            else {
                return eval(&tree[3], global_env, local_env)
            }
        }
        _=>{panic!("The value of condition should be bool")}
    }
}

fn define(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    assert!(tree.len() == 3);
    match &tree[1] {
        RispExp::Symbol(key) => {
            let tem = eval(&tree[2], global_env, local_env);
            global_env.data.insert(key.clone(), tem);
        }
        _ => {
            panic!("variable name type error")
        }
    }

    return RispExp::Nothing();
}


pub fn base_env() -> RispEnv {
    let mut re = RispEnv {
        data: HashMap::new(),
        parent: None,
    };
    re.data
        .insert(String::from("begin"), RispExp::BaseFunc(begin));
    re.data
        .insert(String::from("if"), RispExp::BaseFunc(rif));
    re.data
        .insert(String::from("define"), RispExp::BaseFunc(define));
    re.data
        .insert(String::from("list"), RispExp::BaseFunc(list));
    //  re.data.insert(String::from("lambda"), RispExp::BaseFunc(lambda));
    re.data
        .insert(String::from("+"), RispExp::BaseFunc(operation::add));
    re.data
        .insert(String::from("-"), RispExp::BaseFunc(operation::sub));
    re.data
        .insert(String::from("*"), RispExp::BaseFunc(operation::mul));
    re.data
        .insert(String::from("/"), RispExp::BaseFunc(operation::div));
    re.data
        .insert(String::from("="), RispExp::BaseFunc(operation::eq));
    re.data
        .insert(String::from(">"), RispExp::BaseFunc(operation::gt));
    re.data
        .insert(String::from("<"), RispExp::BaseFunc(operation::lt));
    re.data
        .insert(String::from(">="), RispExp::BaseFunc(operation::geq));
    re.data
        .insert(String::from("<="), RispExp::BaseFunc(operation::leq));
    re.data
        .insert(String::from("pi"), RispExp::Number(3.1416f64));
    return re;
}
