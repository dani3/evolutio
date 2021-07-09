use crate::lexer::Token;

/// Trait that defines the behavior of a `Node`. Every `Node` in the AST has
/// to implement this trait, meainin it has to provide a method that returns
/// the literal value of the token it's associated with.
trait Node {
    /// Returns the literal value of the token it's associated with.
    fn token_literal(&self) -> String;
}

/// Trait that defines the behavior of a `Statement`. `Statement`s don't produce values.
trait Statement {
    /// Dummy method. It's mainly for debugging purposes.
    fn statement_node(&self);
}

/// Trait that defines the behavior of an `Expression`. `Expression`s produce values.
trait Expression {
    /// Dummy method. It's mainly for debugging purposes.
    fn expression_node(&self);
}

/// The`Program` node is going to be the root node of every AST the parser produces.
///
/// Every valid Monkey program is a series of statements.
struct Program<T>
where
    T: Node + Statement,
{
    statements: Vec<T>,
}

impl<T> Node for Program<T>
where
    T: Node + Statement,
{
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            String::default()
        } else {
            self.statements[0].token_literal()
        }
    }
}

struct Identifier<'a> {
    token: Token<'a>,
    value: &'a str,
}

impl Expression for Identifier<'_> {
    fn expression_node(&self) {}
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> String {
        // todo use &str instead if possible
        String::from(self.token.literal)
    }
}

// Node that represents a `Let` statement in the AST.
struct LetStatement<'a, T>
where
    T: Node + Expression,
{
    token: Token<'a>,
    identifier: Identifier<'a>,
    value: T,
}

impl<T> Statement for LetStatement<'_, T>
where
    T: Node + Expression,
{
    fn statement_node(&self) {}
}

impl<T> Node for LetStatement<'_, T>
where
    T: Node + Expression,
{
    fn token_literal(&self) -> String {
        // todo use &str instead if possible
        String::from(self.token.literal)
    }
}
