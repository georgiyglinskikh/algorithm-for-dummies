fn main() {
    let mut expression = String::new();

    std::io::stdin()
        .read_line(&mut expression)
        .expect("cannot read expression");

    let expression = expression.trim_end().chars();

    let mut brackets: Vec<char> = vec![];

    let mut valid = true;

    for ch in expression {
        match ch {
            '{' | '[' | '(' | '<' => brackets.push(ch),
            '}' | ']' | ')' | '>' => {
                let prev = match brackets.pop() {
                    Some(bracket) => bracket,
                    _ => {
                        valid = false;
                        break;
                    }
                };

                if !((prev == '{' && ch == '}')
                    || (prev == '[' && ch == ']')
                    || (prev == '(' && ch == ')')
                    || (prev == '<' && ch == '>'))
                {
                    valid = false;
                    break;
                }
            }
            _ => panic!("unknown type of brackets"),
        }
    }

    println!("{}", valid);
}
