use super::node::Node;

pub struct japi_t {
    pub nodes: Vec<Node>,
}

impl japi_t {
    pub fn new(existing: Vec<Node>) -> japi_t {
        return japi_t {
            nodes: existing,
        };
    }
}