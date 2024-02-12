use crate::expression::{RispEnv, RispExp, Func};
use std::collections::HashMap;
use std::rc::Rc;

// fn lambda(tree: & Vec<RispExp>, _env: & mut RispEnv) -> RispExp{
//     return RispExp::Proc(tree.clone())
// }

// fn proc(tree: & Vec<RispExp>, _env: & mut RispEnv) -> RispExp{
//     return RispExp::Lambda(tree.clone())
// }

fn eval_func_partial(
    tree: &Vec<RispExp>,
    args: &Rc<Vec<String>>,
    proc: &Rc<RispExp>,
    bindings: &Rc<RispEnv>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    // return a Func
    // building the closure, which is a node in the closure environment tree
    let mut vars = HashMap::<String, RispExp>::new();
    for (i, item) in tree.iter().enumerate() {
        if i == 0 {
            continue;
        }
        vars.insert(
            args.get(i - 1).unwrap().clone(),
            eval(item, global_env, local_env),
        );
    }
    let mut new_args: Vec<String> = Vec::with_capacity(args.len() + 1 - tree.len());
    for i in 0..(args.len() + 1 - tree.len()){
        new_args.push(args[tree.len() - 1 + i].clone());
    }
    let new_args = Rc::new(new_args);
    
    let bindings = Rc::new(RispEnv {
        data: vars,
        parent: Some(Rc::clone(bindings)),
    });
    let proc = Func {
        args: Rc::clone(&new_args),
        proc: Rc::clone(proc),
        bindings: bindings,
    };
    RispExp::Func(Rc::new(proc))
}

fn eval_list(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let header = eval(&tree[0], global_env, local_env);
    match header {
        RispExp::BaseFunc(func) => func(tree, global_env, local_env),
        RispExp::Func(pointer) =>{
            let args = &(pointer.args);
            let proc = &(pointer.proc);
            let bindings = &(pointer.bindings);

            let mut vars = HashMap::<String, RispExp>::new();
            if tree.len()-1<args.len(){
                return eval_func_partial(tree, args, proc, bindings, global_env, local_env);
            }
            for (i, item) in tree.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                vars.insert(
                    args.get(i - 1).unwrap().clone(),
                    eval(item, global_env, local_env),
                );
            }
            let local_env = RispEnv {
                data: vars,
                parent: Some(Rc::clone(bindings)),
            };
            let local_env = Rc::new(local_env);
            eval(&(**proc), global_env, Some(&local_env))
        },
        _ => {
            panic!("not implemented")
        }
    }
}

fn eval_lambda(
    args: &Rc<Vec<String>>,
    proc: &Rc<RispExp>,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    // return a Func
    // building the closure, which is a node in the closure environment tree
    let proc = Func {
        args: Rc::clone(args),
        proc: Rc::clone(proc),
        bindings: match local_env {
            Some(p) => Rc::clone(p),
            None => Rc::new(RispEnv {
                data: HashMap::new(),
                parent: None,
            }),
        },
    };
    RispExp::Func(Rc::new(proc))
}

pub fn eval(tree: &RispExp, global_env: &mut RispEnv, local_env: Option<&Rc<RispEnv>>) -> RispExp {
    match tree {
        RispExp::Pointer(pointer) => match &(**pointer) {
            RispExp::List(_) => eval(&(**pointer), global_env, local_env),
            _ => RispExp::Pointer(Rc::clone(pointer)),
        },
        RispExp::Lambda { args, proc } => eval_lambda(&args, &proc, local_env),
        RispExp::Symbol(symbol) => {
            match local_env {
                Some(env) => match env.get(symbol, global_env) {
                    RispExp::BaseFunc(f) => RispExp::BaseFunc(*f),
                    RispExp::Number(f) => RispExp::Number(*f),
                    RispExp::Pointer(pointer) => RispExp::Pointer(Rc::clone(pointer)),
                    _ => {
                        panic!("function not found")
                    }
                },
                None => {
                    // println!("{symbol}");
                    if global_env.data.contains_key(symbol) {
                        let content = global_env.data.get(symbol).unwrap();
                        match content {
                            RispExp::BaseFunc(f) => RispExp::BaseFunc(*f),
                            RispExp::Number(f) => RispExp::Number(*f),
                            RispExp::Func(f) => RispExp::Func(Rc::clone(f)),
                            _ => {
                                panic!("function not found")
                            }
                        }
                    } else {
                        println!("{symbol}");
                        panic!("{symbol} not found")
                    }
                }
            }
        }
        RispExp::Number(k) => RispExp::Number(*k),
        RispExp::List(lis) => return eval_list(lis, global_env, local_env),
        _ => {
            panic!("not implemented")
        }
    }
}