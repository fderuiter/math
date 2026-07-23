#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub(crate) fn parse_latex_math(latex: &str) -> (Vec<String>, Vec<MathOp>) {
    let mut vars = Vec::new();
    let mut ops = Vec::new();

    let mut chars = latex.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '+' => ops.push(MathOp::Add),
            '-' => ops.push(MathOp::Sub),
            '*' => ops.push(MathOp::Mul),
            '/' => ops.push(MathOp::Div),
            '^' => ops.push(MathOp::Pow),
            '\\' => {
                let mut cmd = String::new();
                while let Some(&n) = chars.peek() {
                    if n.is_alphabetic() {
                        cmd.push(n);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match cmd.as_str() {
                    "frac" => ops.push(MathOp::Div),
                    "times" | "cdot" => ops.push(MathOp::Mul),
                    "sum" | "int" => {} // ignore
                    "left" | "right" => {}
                    _ => {
                        // could be a greek letter or a math symbol, ignore for now
                        // or if it's \hat{f}, etc.
                    }
                }
            }
            c if c.is_alphabetic() => {
                let mut var = String::new();
                var.push(c);
                // We'll just collect single character variables usually, or if there are subscripts
                // Actually, let's just collect words.
                while let Some(&n) = chars.peek() {
                    if n.is_alphabetic() {
                        var.push(n);
                        chars.next();
                    } else {
                        break;
                    }
                }
                vars.push(var);
            }
            _ => {}
        }
    }

    // Heuristics for implicit multiplication
    // e.g. "2 B R" -> if we see a number then a var, or var then var, it's Mul.
    // For simplicity, just seeing if there's any operations.
    // We can refine this later if needed.

    (vars, ops)
}
