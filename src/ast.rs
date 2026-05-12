#[derive(Debug, Clone)]
pub enum Node {
    FunctionCall {
        name: String,
        args: Vec<Node>,
    },
    StringLiteral(String),
    Concat(Vec<Node>),
}