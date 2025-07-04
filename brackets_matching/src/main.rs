fn match_arg(arg: &String) {
    let mut vec = vec![];
    for ch in arg.chars() {
        match ch {
            '(' | '[' | '{' => {
                vec.push(ch);
            }
            ')' | ']' | '}' => {
                if let Some(ch2) = vec.pop() {
                    if
                        !(
                            (ch2 == '(' && ch == ')') ||
                            (ch2 == '{' && ch == '}') ||
                            (ch2 == '[' && ch == ']')
                        )
                    {
                        println!("Error");
                        return;
                    }
                }
            }
            _ => {}
        }
    }
    println!("OK")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for arg in &args[1..] {
        match_arg(arg);
    }
}
