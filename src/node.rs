#[derive(Eq, PartialEq)]
#[derive(Clone, Debug)]
pub enum Node { 
    BlockNode(Box<Vec<Node>>),
    ValueNode(String, ObjectType),
    AssignmentNode(String, Box<Node>),
    Null
}

#[derive(Eq, PartialEq)]
#[derive(Clone, Debug)]
pub enum ObjectType {
    String, Number, Bool, Null
}