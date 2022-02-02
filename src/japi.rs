use super::node::Node;
use super::lexer;
use super::parser;

pub struct japi_t {
    pub nodes: Vec<Node>,
}

impl japi_t {
    pub fn new(source: String) -> japi_t {
        let mut lexer = lexer::lexer_t::new(source.as_str());
        lexer.scan_tokens();
    
        /*for token in lexer.tokens.clone() {
            println!("Token: {:?}, {1}, {2}", token.tok_type, token.value, token.line);
        }*/
    
        let mut parser = parser::parser_t::new(lexer.tokens);
        parser.parse();
    
        /*for node in parser.nodes.clone() {
            println!("{:?}", node);
        }*/

        return japi_t {
            nodes: parser.nodes.clone(),
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