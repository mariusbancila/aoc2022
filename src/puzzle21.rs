use crate::utils;
use std::{path::Path, collections::HashMap};

pub fn execute() {
    println!("=== puzzle 21 ===");

    let test_monkeys = read_monkeys("./data/input21test.txt");
    let test_sum = find_root_value(&test_monkeys, &String::from("root"));
    assert_eq!(152, test_sum);

    let monkeys = read_monkeys("./data/input21.txt");
    let sum = find_root_value(&monkeys, &String::from("root"));
    println!("sum={}", sum);    

    println!();
}

#[derive(Copy,Clone)]
enum Operand {
    Plus,
    Minus,
    Mul,
    Div
}

struct Expression {
    left : String,
    right : String,
    op : Operand
}

impl Expression {
    fn new(l : &str, r : &str, o : Operand) -> Expression {
        Expression { left: l.to_string(), right: r.to_string(), op: o }
    }
}

struct Monkey {
    #[allow(unused)]
    name : String,
    value : Option<i64>,
    expr : Option<Expression>    
}

impl Monkey {
    fn from_value(n : &str, v : i64) -> Monkey {
        Monkey { name: n.to_string(), value: Some(v), expr: None }
    }

    fn from_expr(n: &str, l : &str, r : &str, o : Operand) -> Monkey {
        Monkey { name: n.to_string(), value: None, expr: Some(Expression::new(l, r, o)) }
    }
}

type MonkeyGroup = HashMap<String, Monkey>;

fn read_monkeys<P>(filename : P) -> MonkeyGroup
where P : AsRef<Path> {
    let mut monkeys = MonkeyGroup::new();

    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(monkey_line) = line {
                let name = &monkey_line[0..4];
                let expr = &monkey_line[6..];
                if expr.len() > 4 {
                    let left = &expr[0..4];
                    let right = &expr[7..];
                    let c = expr.chars().nth(5).unwrap();
                    let op = match c {
                        '+' => Operand::Plus,
                        '-' => Operand::Minus,
                        '*' => Operand::Mul,
                        '/' => Operand::Div,
                        _ => panic!("Invalid operand")
                    };

                    monkeys.insert(name.to_string(), Monkey::from_expr(name, left, right, op));
                }
                else {
                    let value = expr.parse::<i64>().unwrap();
                    monkeys.insert(name.to_string(), Monkey::from_value(name, value));
                }
            }
        }
    }    

    monkeys
}

fn find_root_value(monkeys : &MonkeyGroup, name : &String) -> i64 {
    if let Some(monkey) = monkeys.get(name) {
        if let Some(value) = monkey.value {
            return value;
        }
        else if let Some(expr) = &monkey.expr {
            return evaluate(
                find_root_value(monkeys, &expr.left), 
                find_root_value(monkeys, &expr.right), 
                expr.op);
        }
    } 

    0
}

fn evaluate(left : i64, right : i64, op : Operand) -> i64 {
    match op {
        Operand::Plus => left + right,
        Operand::Minus => left - right,
        Operand::Mul => left * right,
        Operand::Div => left / right
    }
}