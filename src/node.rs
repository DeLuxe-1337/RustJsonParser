#[derive(Eq, PartialEq)]
#[derive(Clone, Debug)]
pub enum Node { 
    BlockNode(Box<Vec<Node>>),
    ValueNode(String, ObjectType),
    AssignmentNode(String, Box<Node>),
    Null
}

#[derive(Eq, PartialEq)]
#[derive(Clone)]
pub enum ObjectType {
    String(String), 
    Number(i32), 
    Bool(bool), 
    Null
}

impl ObjectType {
    pub fn as_int(self) -> i32 {
        match self { 
            ObjectType::Number(n) => n,
            _ => 0
        }
    }
    pub fn as_string(self) -> String {
        match self { 
            ObjectType::String(s) => s,
            _ => "null".to_string()
        }
    }
    pub fn as_bool(self) -> bool {
        match self { 
            ObjectType::Bool(b) => b,
            _ => false
        }
    }
}

use std::fmt;

impl fmt::Debug for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::String(s) => write!(f, "{}", s),
            ObjectType::Number(n) => write!(f, "{}", n),
            ObjectType::Bool(b) => write!(f, "{}", b),
            ObjectType::Null => write!(f, "null")
        }
    }
}