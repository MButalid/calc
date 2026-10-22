use std::num::ParseIntError;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// `Expr` represents any mathematical expression, from a simple number to a
/// complex, nested expression.
///
/// **Recursive Data Structures:** Why use `Square(Box<Expr>)` instead of
/// `Square(Expr)`?
///
/// Rust must know the size of every type at compile time. If `Expr` directly
/// contained another `Expr`, it would have an infinitely recursive size,
/// because each `Square` would need to contain another complete `Expr`.
///
/// Wrapping `Expr` in `Box` introduces indirection. A `Box<Expr>` has a
/// fixed size because it stores a pointer to an `Expr` allocated on the heap.
/// This allows `Expr` to be recursive without having an infinitely sized type.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Square(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NotANumber(String),
    MissingOperand,
}

pub enum EvalError {}

// Takes a string and produces an expression tree
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();
    for word in input.split_ascii_whitespace() {
        match word {
            "+" => {
                let a = stack.pop().ok_or(ParseError::MissingOperand)?;
                let b = stack.pop().ok_or(ParseError::MissingOperand)?;
                let a = Box::new(a);
                let b = Box::new(b);
                let expr = Expr::Add(a, b);
                stack.push(expr);
            },
            _ => {
                let number = word
                    .parse::<i64>()
                    .map_err(|_| ParseError::NotANumber(word.to_string()))?;
                    let expr = Expr::Number(number);
                    stack.push(expr);
            }
        }
    };
    assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(res)
}

// Evaluates the expression
pub fn eval(expr: &Expr) -> Result<i64, ParseError>  {
    match expr {
        // n: &i64
        Expr::Number(n) => Ok(*n),
        Expr::Add(a,  b) => {
            let a = eval(a)?;
            let b = eval(b)?;
            Ok(a + b)
        },
        _ => Ok(1)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let input = "42";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn not_a_number() {
        let input = "X";
        let erorr = parse(input).unwrap_err();
        assert_eq!(erorr, ParseError::NotANumber("X".to_string()));
    }

    #[test]
    fn add() {
        let input = "40 2 +";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }


}
