
#[derive(Clone, Debug)]
pub enum Node { 
    BlockNode {
        statements: Option<Box<Vec<Node>>>,
    },
    ValueNode {
        value: String,
    },
    AssignmentNode {
        name: String,
        value: Option<Box<Node>>,
    },
}