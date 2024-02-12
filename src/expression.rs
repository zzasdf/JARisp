use std::collections::HashMap;
use std::rc::Rc;
#[derive(Clone, Debug)]
pub enum RispExp {
    Symbol(String),
    Number(f64),
    Bool(bool),
    List(Vec<RispExp>),
    BaseFunc(fn(&Vec<RispExp>, &mut RispEnv, Option<&Rc<RispEnv>>) -> RispExp),
    // lambda in the ast
    Lambda {
        args: Rc<Vec<String>>,
        proc: Rc<RispExp>,
    },
    // for closure
    Func(Rc<Func>),
    // to avoid large variable copy, like a ast tree and environment
    Pointer(Rc<RispExp>),
    Nothing(),
}
// 我想用eval实现lambda的函数的调用
// 这样需要保留一个完整的RispExp
// 我想用eval实现lambda的函数的调用

#[derive(Debug)]
enum RispErr {
    Reason(String),
}

#[derive(Clone, Debug)]
pub struct Func {
    pub args: Rc<Vec<String>>,
    pub proc: Rc<RispExp>,
    pub bindings: Rc<RispEnv>,
}

#[derive(Clone, Debug)]
pub struct RispEnv {
    pub data: HashMap<String, RispExp>,
    pub parent: Option<Rc<RispEnv>>,
}

impl RispEnv {
    fn get_local(&self, s: &String) -> Option<&RispExp> {
        if self.data.contains_key(s) {
            return self.data.get(s);
        } else {
            match &self.parent {
                Some(p) => p.get_local(s),
                _ => None,
            }
        }
    }
    pub fn get<'a>(&'a self, s: &String, env: &'a RispEnv) -> &'a RispExp {
        match self.get_local(s) {
            Some(k) => k,
            None => env.data.get(s).unwrap(),
        }
    }
}
