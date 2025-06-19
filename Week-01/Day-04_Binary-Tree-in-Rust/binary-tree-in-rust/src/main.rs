use binary_tree_in_rust::Node;

fn main() {
    let mut head = Node::new(5);
    head.insert(7);
    head.insert(3);
    head.insert(4);
    head.insert(8);
    head.insert(6);
    head.insert(1);

    println!("final tree: {:#?}", head);
}
