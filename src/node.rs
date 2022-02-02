#[derive(Eq, PartialEq)]
#[derive(Clone, Debug)]
pub enum Node { 
    BlockNode(Box<Vec<Node>>),
    ValueNode(String),
    AssignmentNode(String, Box<Node>),
    Null
}