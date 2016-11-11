use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap
}

enum NodeType {
    Text(String),
    Element(ElementData)
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs
        })
    }
}

fn main() {
    unimplemented!()
}
