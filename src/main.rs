// use std::io::Write;
use std::fs;
mod expression;
mod global_env;
mod operation;
mod eval;
use expression::RispExp;
use std::rc::Rc;

fn parse_args(line: & Vec<&str>, b: usize) -> (Vec<String>, usize) {
    assert_eq!(line[b], "(");
    let mut index = b+1;
    let mut re = Vec::<String>::new();
    while index < line.len() {
        assert_ne!(line[index], "(");
        assert_ne!(line[index], "lambda");
        assert_ne!(line[index], "define");
        if line[index] != ")"{
            re.push(String::from(line[index]));
            index+=1;
        }
        else{
            return (re, index+1);
        }
    }
    panic!("syntax error")
}
fn parse_lambda(line: & Vec<&str>, b: usize) -> (RispExp, usize) {
    // parse lambda: lambda args proc("(",")" not included)
    // line: input lines
    // b: start index of the parsing
    // e: the stop point of the parsing + 1
    assert_eq!(line[b], "lambda");
    let index = b + 1;
    let (args,index) = parse_args(line, index);
    let (sub_tree, index) = parse_list(line, index);
    let re_lambda = RispExp::Lambda { args: Rc::new(args), proc: Rc::new(sub_tree) };
    return (re_lambda, index);
}

fn parse_list(line: & Vec<&str>, b: usize) -> (RispExp, usize) {
    // line: input lines
    // b: start index of the parsing
    // e: the stop point of the parsing + 1
    assert_eq!(line[b], "(");
    let mut index = b + 1;
    let mut tree = Vec::<RispExp>::new();
    while index < line.len() {
        if line[index].parse::<f64>().is_ok() {
            assert_ne!(tree.len(), 0);
            tree.push(RispExp::Number(line[index].parse::<f64>().unwrap()));
            index += 1;
        } else if line[index] == "(" {
            let (sub_tree, new_index) = parse_list(line, index);
            index = new_index;
            tree.push(sub_tree);
        } else if line[index] == "lambda" {
            let (lambda_tree, index) = parse_lambda(line, index);
            assert!(index<line.len());
            assert_eq!(line[index],")");
            return (lambda_tree, index+1);
        } else if line[index] != ")" {
            tree.push(RispExp::Symbol(String::from(line[index])));
            index += 1;
        } else {
            break;
        }
    }
    assert!(index<line.len());
    return (RispExp::Pointer(Rc::new(RispExp::List(tree))), index + 1);
}

fn main() {
    let line = fs::read_to_string("examples\\test1.txt").unwrap();
    // let line = &line[..line.len() - 2] // only necessary when input from teminal
    //     .replace("(", " ( ")
    //     .replace(")", " ) ");
    let line = &line.replace("(", " ( ") .replace(")", " ) ").replace("\n", " ").replace("\r", " ");
    let tem: Vec<&str> = line.split(' ').collect();
    let tem: Vec<&str> = tem.into_iter().filter(|x| x.len() > 0).collect();
    let (tree, _) = parse_list(&tem, 0);
    println!("{:?}", tree);
    let mut env = global_env::base_env();
    match eval::eval(&tree, &mut env, None) {
        RispExp::Number(k) => {
            println!("{}",k);
        }
        _=>{ }
    }
}
