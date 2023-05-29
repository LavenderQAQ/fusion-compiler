use self::lexer::Token;

pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) {
        let mut printer = ASTPrinter { indent: 0 };
        self.visit(&mut printer)
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
        }
    }
    fn visit_statement(&mut self, statement: &ASTStatement);
    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => self.visit_number(number),
            ASTExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            ASTExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
        }
    }
    fn visit_expression(&mut self, expression: &ASTExpression);
    fn visit_number(&mut self, expression: &ASTNumberExpression);
    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression);
    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression,
    );
}

pub struct ASTPrinter {
    indent: usize,
}

const LEVEL_INDENT: usize = 2;

impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_with_indent("Statement:");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_statement(self, statement);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.print_with_indent("Expression:");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_expression(self, expression);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_number(&mut self, expression: &ASTNumberExpression) {
        self.print_with_indent(&format!("Number: {}", expression.number));
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.print_with_indent("Binary Expression:");
        self.indent += LEVEL_INDENT;
        self.print_with_indent(&format!("Operator: {:?}", binary_expression.operator.kind));
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression,
    ) {
        self.print_with_indent("Parenthesized Expression:");
        self.indent += LEVEL_INDENT;
        self.visit_expression(&parenthesized_expression.expression);
        self.indent -= LEVEL_INDENT;
    }
}

impl ASTPrinter {
    fn print_with_indent(&mut self, text: &str) {
        for _ in 0..self.indent {
            print!("  ");
        }
        println!("{}", text);
    }
}

pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTStatement {
    kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        Self { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        Self::new(ASTStatementKind::Expression(expr))
    }
}

#[derive(Debug)]
pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
}

#[derive(Debug)]
pub struct ASTNumberExpression {
    number: i64,
}

#[derive(Debug)]
pub enum ASTBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct ASTBinaryOperator {
    kind: ASTBinaryOperatorKind,
    token: Token,
}

impl ASTBinaryOperator {
    fn new(kind: ASTBinaryOperatorKind, token: Token) -> Self {
        Self { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Plus | ASTBinaryOperatorKind::Minus => 1,
            ASTBinaryOperatorKind::Multiply | ASTBinaryOperatorKind::Divide => 2,
        }
    }
}

#[derive(Debug)]
pub struct ASTBinaryExpression {
    left: Box<ASTExpression>,
    operator: ASTBinaryOperator,
    right: Box<ASTExpression>,
}

#[derive(Debug)]
pub struct ASTParenthesizedExpression {
    expression: Box<ASTExpression>,
}

#[derive(Debug)]
pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        Self { kind }
    }

    pub fn number(number: i64) -> Self {
        Self::new(ASTExpressionKind::Number(ASTNumberExpression { number }))
    }

    pub fn binary(left: ASTExpression, operator: ASTBinaryOperator, right: ASTExpression) -> Self {
        Self::new(ASTExpressionKind::Binary(ASTBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    pub fn parenthesized(expression: ASTExpression) -> Self {
        Self::new(ASTExpressionKind::Parenthesized(
            ASTParenthesizedExpression {
                expression: Box::new(expression),
            },
        ))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_learn() {
        let vec = vec![1, 2, 3, 4, 5];
        println!("{:?}", vec.get(2))
    }
}
