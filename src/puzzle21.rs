use crate::utils;
use std::{path::Path, collections::HashMap, cmp::Ordering};

pub fn execute() {
    println!("=== puzzle 21 ===");

    let test_monkeys = read_monkeys("./data/input21test.txt");
    let test_sum = find_root_value(&test_monkeys, &String::from("root"));
    assert_eq!(152, test_sum);

    let test_monkeys2 = read_monkeys("./data/input21test2.txt");
    let test_sum2 = find_root_value(&test_monkeys2, &String::from("root"));
    println!("test sum={}", test_sum2);

    let monkeys = read_monkeys("./data/input21.txt");
    let sum = find_root_value(&monkeys, &String::from("root"));
    println!("sum={}", sum);

    let test_hum_value = find_humn_value(&test_monkeys, &String::from("root"), &String::from("humn"));
    assert_eq!(301, test_hum_value);

    //let test_hum_value2 = find_humn_value(&test_monkeys2, &String::from("root"), &String::from("humn"));
    //println!("test human={}", test_hum_value2);

    let hum_value = find_humn_value(&monkeys, &String::from("root"), &String::from("humn"));
    println!("human={}", hum_value);

    println!();
}

#[derive(Copy,Clone,PartialEq,Eq)]
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

fn find_humn_value(monkeys : &MonkeyGroup, root_name : &String, humn_name : &String) -> i64 {
    let root = monkeys.get(root_name).unwrap();

    if let Some(root_expr) = &root.expr {
        let is_humn_on_left = contains_humn(monkeys, &root_expr.left, &humn_name);

        if is_humn_on_left {
            // find the right tree value
            let value = find_root_value(&monkeys, &root_expr.right);

            // continue computing on the left
            return compute_humn_value(&monkeys, &root_expr.left, &humn_name, value);
        } 
        else {
            // find the left tree value
            let value = find_root_value(&monkeys, &root_expr.left);

            // continue computing on the right
            return compute_humn_value(&monkeys, &root_expr.right, &humn_name, value);
        };
    }

    0
}

fn compute_humn_value(monkeys : &MonkeyGroup, root_name : &String, humn_name : &String, root_value : i64) -> i64 {
    // if we reached the target node than the root value is the result    
    if root_name.cmp(humn_name) == Ordering::Equal {
        return root_value;
    }

    // get the root node
    let root = monkeys.get(root_name).unwrap();

    if let Some(root_expr) = &root.expr {
        // check where the human node is
        let is_humn_on_left = contains_humn(monkeys, &root_expr.left, &humn_name);                  

        return if is_humn_on_left {
            // compute the value of the other sub-tree
            let other_value = find_root_value(&monkeys, &root_expr.right);

            //  compute the value of the sub-tree where the human is
            let human_tree_value = match root_expr.op {
                Operand::Plus => root_value - other_value,
                Operand::Minus => root_value + other_value,
                Operand::Mul => root_value / other_value,
                Operand::Div => root_value * other_value            
            };

            // continue recursion with the sub-tree where the human is
            compute_humn_value(monkeys, &root_expr.left, &humn_name, human_tree_value)
        }
        else {
            let other_value = find_root_value(&monkeys, &root_expr.left);

            let human_tree_value = match root_expr.op {
                Operand::Plus => root_value - other_value,
                Operand::Minus => other_value - root_value,
                Operand::Mul => root_value / other_value,
                Operand::Div => other_value / root_value            
            };

            compute_humn_value(monkeys, &root_expr.right, &humn_name, human_tree_value)
        }
    }
    else {
        panic!("wrong path!");
    }
}

fn contains_humn(monkeys : &MonkeyGroup, root_name : &String, humn_name : &String) -> bool {
    if root_name.cmp(humn_name) == Ordering::Equal {
        return true;
    }

    if let Some(monkey) = monkeys.get(root_name) {
        if let Some(_) = monkey.value {
            return false;
        }
        else if let Some(expr) = &monkey.expr {
            return  contains_humn(monkeys, &expr.left, &humn_name) ||
                    contains_humn(monkeys, &expr.right, &humn_name);
        }
    } 

    false
}

fn evaluate(left : i64, right : i64, op : Operand) -> i64 {
    match op {
        Operand::Plus => left + right,
        Operand::Minus => left - right,
        Operand::Mul => left * right,
        Operand::Div => left / right
    }
}