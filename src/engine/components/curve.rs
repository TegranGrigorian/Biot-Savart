use nalgebra::Vector3;

#[derive(Debug, Clone, PartialEq)]
// Expressions for token object
enum Token {
    Num(f64),
    T,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Ident(String),
}

fn tokenize(src: &str) -> Result<Vec<Token>, String> { // a whole lotta expressions
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' => { chars.next(); }
            '+' => { tokens.push(Token::Plus);   chars.next(); }
            '-' => { tokens.push(Token::Minus);  chars.next(); }
            '*' => { tokens.push(Token::Star);   chars.next(); }
            '/' => { tokens.push(Token::Slash);  chars.next(); }
            '^' => { tokens.push(Token::Caret);  chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '0'..='9' | '.' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                    } else if c == 'e' || c == 'E' {
                        num.push(c);
                        chars.next();
                        if let Some(&sign) = chars.peek() {
                            if sign == '+' || sign == '-' {
                                num.push(sign);
                                chars.next();
                            }
                        }
                    } else {
                        break;
                    }
                }
                let val: f64 = num.parse().map_err(|_| format!("invalid number: {num}"))?;
                tokens.push(Token::Num(val));
            }
            'a'..='z' | 'A'..='Z' | '_' => { // find the char
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if ident == "t" {
                    tokens.push(Token::T);
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            _ => return Err(format!("unexpected character: '{ch}'")),
        }
    }
    Ok(tokens)
}

#[derive(Debug, Clone)]
enum Expr {
    Num(f64),
    T,
    BinOp { op: char, lhs: Box<Expr>, rhs: Box<Expr> },
    UnaryMinus(Box<Expr>),
    Call { name: String, arg: Box<Expr> },
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser { // not going ot comment this much but this parses the string and gets the different operations for it
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next_tok(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.next_tok() {
            Some(ref tok) if tok == expected => Ok(()),
            Some(tok) => Err(format!("expected {expected:?}, got {tok:?}")),
            None => Err(format!("expected {expected:?}, got end of input")),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.next_tok();
                    let rhs = self.parse_mul_div()?;
                    lhs = Expr::BinOp { op: '+', lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Some(Token::Minus) => {
                    self.next_tok();
                    let rhs = self.parse_mul_div()?;
                    lhs = Expr::BinOp { op: '-', lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.next_tok();
                    let rhs = self.parse_unary()?;
                    lhs = Expr::BinOp { op: '*', lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Some(Token::Slash) => {
                    self.next_tok();
                    let rhs = self.parse_unary()?;
                    lhs = Expr::BinOp { op: '/', lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some(Token::Minus) = self.peek() {
            self.next_tok();
            let inner = self.parse_unary()?;
            return Ok(Expr::UnaryMinus(Box::new(inner)));
        }
        self.parse_pow()
    }

    fn parse_pow(&mut self) -> Result<Expr, String> {
        let base = self.parse_atom()?;
        if let Some(Token::Caret) = self.peek() {
            self.next_tok();
            let exp = self.parse_unary()?;
            return Ok(Expr::BinOp { op: '^', lhs: Box::new(base), rhs: Box::new(exp) });
        }
        Ok(base)
    }
    // special chars, e and pi
    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.peek().cloned() {
            Some(Token::Num(n)) => { self.next_tok(); Ok(Expr::Num(n)) }
            Some(Token::T)      => { self.next_tok(); Ok(Expr::T) }
            Some(Token::Ident(name)) => {
                self.next_tok();
                match name.as_str() {
                    "pi" => Ok(Expr::Num(std::f64::consts::PI)),
                    "e"  => Ok(Expr::Num(std::f64::consts::E)),
                    _ => {
                        self.expect(&Token::LParen)?;
                        let arg = self.parse_expr()?;
                        self.expect(&Token::RParen)?;
                        Ok(Expr::Call { name, arg: Box::new(arg) })
                    }
                }
            }
            Some(Token::LParen) => {
                self.next_tok();
                let inner = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(inner)
            }
            Some(tok) => Err(format!("unexpected token: {tok:?}")),
            None => Err("unexpected end of expression".to_string()),
        }
    }
}

fn eval(expr: &Expr, t: f64) -> Result<f64, String> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::T => Ok(t),
        Expr::UnaryMinus(inner) => Ok(-eval(inner, t)?),
        Expr::BinOp { op, lhs, rhs } => {
            let l = eval(lhs, t)?;
            let r = eval(rhs, t)?;
            match op {
                '+' => Ok(l + r),
                '-' => Ok(l - r),
                '*' => Ok(l * r),
                '/' => {
                    if r.abs() < 1.0e-300 {
                        Err("division by zero".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
                '^' => Ok(l.powf(r)),
                _   => Err(format!("unknown op '{op}'")),
            }
        }
        Expr::Call { name, arg } => { // special operations ur welcome :)
            let v = eval(arg, t)?;
            match name.as_str() {
                "sin"   => Ok(v.sin()),
                "cos"   => Ok(v.cos()),
                "tan"   => Ok(v.tan()),
                "sqrt"  => if v < 0.0 { Err(format!("sqrt of negative: {v}")) } else { Ok(v.sqrt()) },
                "abs"   => Ok(v.abs()),
                "exp"   => Ok(v.exp()),
                "ln"    => if v <= 0.0 { Err(format!("ln of non-positive: {v}")) } else { Ok(v.ln()) },
                "log"   => if v <= 0.0 { Err(format!("log of non-positive: {v}")) } else { Ok(v.log10()) },
                "floor" => Ok(v.floor()),
                "ceil"  => Ok(v.ceil()),
                _       => Err(format!("unknown function '{name}'")),
            }
        }
    }
}

pub struct CompiledExpr(Expr);

impl CompiledExpr { // recompiled expression for the engine to use and calc with
    pub fn compile(src: &str) -> Result<Self, String> {
        if src.trim().is_empty() {
            return Err("expression is empty".to_string());
        }
        let tokens = tokenize(src)?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr()?;
        if parser.pos < parser.tokens.len() {
            return Err(format!(
                "unexpected token after expression: {:?}",
                parser.tokens[parser.pos]
            ));
        }
        Ok(CompiledExpr(expr))
    }

    pub fn eval_at(&self, t: f64) -> Result<f64, String> {
        eval(&self.0, t)
    }
}

pub struct ParametricCurve { // parameterizd to var t
    pub name: String,
    x: CompiledExpr,
    y: CompiledExpr,
    z: CompiledExpr,
    pub t_min: f64,
    pub t_max: f64,
}

impl ParametricCurve {
    pub fn new(
        name: String,
        x_expr: &str,
        y_expr: &str,
        z_expr: &str,
        t_min: f64,
        t_max: f64,
    ) -> Result<Self, String> {
        if t_max <= t_min {
            return Err("t_max must be greater than t_min".to_string());
        }
        Ok(ParametricCurve {
            name,
            x: CompiledExpr::compile(x_expr).map_err(|e| format!("x(t): {e}"))?,
            y: CompiledExpr::compile(y_expr).map_err(|e| format!("y(t): {e}"))?,
            z: CompiledExpr::compile(z_expr).map_err(|e| format!("z(t): {e}"))?,
            t_min,
            t_max,
        })
    }

    pub fn sample(&self, n: usize) -> Result<Vec<Vector3<f32>>, String> {
        if n < 2 {
            return Err("samples must be >= 2".to_string());
        }
        let mut points = Vec::with_capacity(n);
        let dt = (self.t_max - self.t_min) / (n as f64 - 1.0);
        for i in 0..n {
            let t = self.t_min + dt * i as f64;
            let px = self.x.eval_at(t)? as f32;
            let py = self.y.eval_at(t)? as f32;
            let pz = self.z.eval_at(t)? as f32;
            if !px.is_finite() || !py.is_finite() || !pz.is_finite() {
                return Err(format!("non-finite value at t = {t:.4}"));
            }
            points.push(Vector3::new(px, py, pz));
        }
        Ok(points)
    }
}

// pub struct Curve {
//     pub name: String,
//     pub points: Vec<Vector3<f32>>,
// }

// impl Curve {
//     const MATH_EXPRESSIONS: &[&str] = &["+", "-", "/", "*", "="];
//     pub fn new(name: String, points: Vec<Vector3<f32>>) -> Curve {
//         Curve { name, points }
//     }

//     pub fn strip_space_equation(equation: &String) -> String {
//         let parts: Vec<&str> = equation.split(' ').collect();
//         String::from_iter(parts)
//     }

//     pub fn strip_parts_equation(equation: &String) {
//         let mut parts: Vec<String> = vec![];
//         let mut current_part = String::new();
//         for ch in equation.chars() {
//             if Self::MATH_EXPRESSIONS.contains(&ch.to_string().as_str()) {
//                 if !current_part.is_empty() {
//                     parts.push(current_part.clone());
//                 }
//                 parts.push(ch.to_string());
//                 current_part.clear();
//             } else if ch != ' ' {
//                 current_part.push(ch);
//             }
//         }
//         if !current_part.is_empty() {
//             parts.push(current_part);
//         }
//         println!("{:?}", parts);
//     }
// }