use std::io;

fn main() {
    let mut finished = false;

    println!("\nThompson's Construction!\n");
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
        
        print_reg_ex(&reg_ex);
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

fn print_reg_ex(reg_ex: &str){
    println!("Printing the FA");
    let mut char_before: char = ' ';
    for (i, c) in reg_ex.chars().enumerate() {
        if i == 0 {
            char_before = c;
        } else { 
            match c {
                '|' => println!("Alter"),
                '*' => println!("Closure"),
                _ => concatenation(char_before, c, i) 
            }
        }
    }
}

fn concatenation(c0: char, c1: char, i: usize) {
    let eps = 'ε';
    print!(" -> S_{0} ->{c0} S_{1} ->{eps} S_{2} ->{c1}", i-1, i, i + 1);
}
    
