

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
    Sub(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NotANumber(String),
    MissingOperand,
    TooManyOperands,
    EmptyInput,
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    DivisionByZero,
}

// Takes a string and produces an expression tree
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Box<Expr>> = Vec::new();
    for word in input.split_ascii_whitespace() {
        let expr = match word {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().ok_or(ParseError::MissingOperand)?;
                let a = stack.pop().ok_or(ParseError::MissingOperand)?;
                match word {
                    "+" => Expr::Add(a, b),
                    "-" => Expr::Sub(a, b),
                    "*" => Expr::Multiplication(a, b),
                    "/" => Expr::Div(a, b),
                    _ => unreachable!(),
                }
            }
            "sqr"  => 
            {
                let a = stack.pop().ok_or(ParseError::MissingOperand)?;
                Expr::Square(a)
            }
            _ => {
                let number = word
                    .parse::<i64>()
                    .map_err(|_| ParseError::NotANumber(word.to_string()))?;
                    Expr::Number(number)
            }
        };
        stack.push(Box::new(expr));
    };

    match stack.pop() {
        Some(expr) if stack.is_empty() => Ok(*expr),
        Some(_) => Err(ParseError::TooManyOperands),
        None => Err(ParseError::EmptyInput),
    }
}

// Evaluates the expression
pub fn eval(expr: &Expr) -> Result<i64, EvalError>  {
    let value = match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => eval(a)? + eval(b)?,
        Expr::Square(a) => eval(a)?.pow(2),
        Expr::Sub(a, b) => eval(a)? - eval(b)?,
        Expr::Multiplication(a, b) => eval(a)? * eval(b)?,
        Expr::Div(_,divisor) if eval(&divisor)? == 0 => {
            return Err(EvalError::DivisionByZero)
        },
        Expr::Div(a, b) => eval(a)? / eval(b)?,
    };
    Ok(value)
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

    #[test]
    fn square() {
        let input = "5 sqr";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 25)
    }

    #[test]
    fn too_many_operands() {
        let input = "42 42 42 +";
        let error = parse(input).unwrap_err();
        assert_eq!(error, ParseError::TooManyOperands);
    }

    #[test]
    fn empty_input() {
        let input = "       ";
        let error = parse(input).unwrap_err();
        assert_eq!(error, ParseError::EmptyInput);
    }


    #[test]
    fn sub() {
        let input = "42 2 -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 40);
    }

    #[test]
    fn mul() {
        let input = "21 2 *";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn div() {
        let input = "84 2 /";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn divide_by_zero() {
        let input = "42 0 /";
        let expr = parse(input).unwrap();
        let error = eval(&expr).unwrap_err();
        assert_eq!(error, EvalError::DivisionByZero);
    }

}
