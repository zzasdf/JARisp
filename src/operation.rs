use crate::expression::{RispEnv, RispExp};
use crate::eval::eval;
use std::rc::Rc;
pub fn add(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let mut re = 0f64;
    for (i, item) in tree.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let tem = eval(item, global_env, local_env);
        let tem = match &tem {
            RispExp::Number(k) => *k,
            _ => {
                panic!("type error")
            }
        };
        if i == 1 {
            re = tem;
        } else {
            re = re + tem;
        }
    }
    return RispExp::Number(re);
}

pub fn sub(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let mut re = 0f64;
    for (i, item) in tree.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let tem = eval(item, global_env, local_env);
        let tem = match &tem {
            RispExp::Number(k) => *k,
            _ => {
                panic!("type error")
            }
        };
        if i == 1 {
            re = tem;
        } else {
            re = re - tem;
        }
    }
    return RispExp::Number(re);
}

pub fn mul(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let mut re = 0f64;
    for (i, item) in tree.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let tem = eval(item, global_env, local_env);
        let tem = match &tem {
            RispExp::Number(k) => *k,
            _ => {
                panic!("type error")
            }
        };
        if i == 1 {
            re = tem;
        } else {
            re = re * tem;
        }
    }
    return RispExp::Number(re);
}

pub fn div(
    tree: &Vec<RispExp>,
    global: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let mut re = 0f64;
    for (i, item) in tree.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let tem = eval(item, global, local_env);
        let tem = match &tem {
            RispExp::Number(k) => *k,
            _ => {
                panic!("type error")
            }
        };
        if i == 1 {
            re = tem;
        } else {
            re = re / tem;
        }
    }
    return RispExp::Number(re);
}

pub fn eq(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let op1 = eval(&tree[1], global_env, local_env);
    let op2 = eval(&tree[2], global_env, local_env);
    match (op1, op2) {
        (RispExp::Number(x),RispExp::Number(y)) => {
            return RispExp::Bool(x==y);
        }
        _=>{panic!("not supported compare type")}
    }
}

pub fn leq(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let op1 = eval(&tree[1], global_env, local_env);
    let op2 = eval(&tree[2], global_env, local_env);
    match (op1, op2) {
        (RispExp::Number(x),RispExp::Number(y)) => {
            return RispExp::Bool(x<=y);
        }
        _=>{panic!("not supported compare type")}
    }
}

pub fn geq(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let op1 = eval(&tree[1], global_env, local_env);
    let op2 = eval(&tree[2], global_env, local_env);
    match (op1, op2) {
        (RispExp::Number(x),RispExp::Number(y)) => {
            return RispExp::Bool(x>=y);
        }
        _=>{panic!("not supported compare type")}
    }
}

pub fn lt(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let op1 = eval(&tree[1], global_env, local_env);
    let op2 = eval(&tree[2], global_env, local_env);
    match (op1, op2) {
        (RispExp::Number(x),RispExp::Number(y)) => {
            return RispExp::Bool(x<y);
        }
        _=>{panic!("not supported compare type")}
    }
}

pub fn gt(
    tree: &Vec<RispExp>,
    global_env: &mut RispEnv,
    local_env: Option<&Rc<RispEnv>>,
) -> RispExp {
    let op1 = eval(&tree[1], global_env, local_env);
    let op2 = eval(&tree[2], global_env, local_env);
    match (op1, op2) {
        (RispExp::Number(x),RispExp::Number(y)) => {
            return RispExp::Bool(x>y);
        }
        _=>{panic!("not supported compare type")}
    }
}
