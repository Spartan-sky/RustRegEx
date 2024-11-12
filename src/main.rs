use std::io;
use std::mem;

pub struct NFA {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: char,
    next: Link,
}

impl NFA {
    pub fn new() -> Self {
        NFA { head : Link::Empty }
    }

    pub fn push(&mut self, elem: char) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, NFA::Empty),
        });

        self.head = Link::More(new_node);
    }
}

fn main() {
    let mut finished = false;
    let mut start_state: State;
    start_state.content = 'a';

    println!("\nRustRegEx!\n");
    while !finished {
        println!("Enter a RegEx to be converted to an NFA: ");

        let mut reg_ex = String::new();

        io::stdin()
            .read_line(&mut reg_ex)
            .expect("Failed to read line");

        let reg_ex = reg_ex.trim();

        println!("You inputed: {}", reg_ex); 

        if !is_valid_reg_ex(&reg_ex){
            continue;
        }


        finished = !finished;
        
//        print_reg_ex(&reg_ex, &num_of_special_char);
    }
}

fn is_valid_reg_ex(reg_ex: &str) -> bool {
    if !is_valid_start(reg_ex.chars().next().unwrap()) {
        return false;
    }

    if !is_balanced_paranthesis(&reg_ex) {
        println!("Inbalanced paranthesis.");
        return false;
    }

    if !is_alter_valid(&reg_ex) {
        println!("Invalid |");
        return false;
    }
    
    true
}

fn is_valid_start(c: char) -> bool {
    if c == '|' || c == '*' {
        println!("Cannot start RegEx with {c}");
        return false;
    }

    true
}

fn is_balanced_paranthesis(reg_ex: &str) -> bool {
    let mut paran_stack: Vec<char> = Vec::new();

    for c in reg_ex.chars() {
        if c == '(' {
            paran_stack.push(c);
        }
        if c == ')' {
            if paran_stack.is_empty() {
                return false;
            }
            paran_stack.pop();
        }
    }

    if !paran_stack.is_empty() {
        return false;
    }

    true
}

fn is_alter_valid(reg_ex: &str) -> bool {
    if reg_ex.contains("(|") || reg_ex.contains("|)") || reg_ex.contains("*|") || reg_ex.contains("|*") {
        return false;
    }
    true
}

