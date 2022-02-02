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

    fn search_node(&self, current: &Node, name: &str) -> Node {
        match current {
            Node::BlockNode(Block) => {
                for new_node in *Block.clone() {
                    let search = self.search_node(&new_node, name);

                    if search != Node::Null {
                        return search;
                    }
                }
            },
            Node::AssignmentNode(asname, eqto) => {
                if asname == name {
                    return *eqto.clone();
                } 
            }
            _ => {

            }
        }

        return Node::Null;
    }

    pub fn index(&self, index: &str) -> Node {
        let go = index.split('>').collect::<Vec<&str>>();
        let mut current: Node = self.nodes[0].clone();

        for select in go {
            current = self.search_node(&current, select);
        }

        return current;
    }
}