use id_tree::InsertBehavior::*;
use id_tree::*;

const INPUT: &str = include_str!("../../day07.txt");

#[derive(Clone, Debug, Default)]
struct FsItem {
    fstype: FsType,
    size: usize,
    name: String,
}

#[derive(Clone, Debug, Default)]
enum FsType {
    #[default]
    Dir,
    File,
}

fn main() {
    let mut lines = INPUT.lines();

    let mut tree: Tree<FsItem> = TreeBuilder::new().build();
    let root_id: NodeId = tree.insert(Node::new(FsItem::default()), AsRoot).unwrap();
    let mut current_dir = root_id.clone();
    let _ = lines.next(); // throw away `cd /`

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        println!("line: {}", line.unwrap());
        let split: Vec<&str> = line.unwrap().split(' ').collect();
        let change_dir = match split[0] {
            "$" => command(&mut tree, &current_dir, split),
            "dir" => None, //todo!("dir"),
            _ => add_file(&mut tree, &current_dir, split),
        };

        if let Some(d) = change_dir {
            current_dir = d;
        }
    }

    for node in tree.traverse_pre_order(&root_id).unwrap() {
        println!("{:?}, ", node.data());
    }
}

fn command(tree: &mut Tree<FsItem>, node: &NodeId, input: Vec<&str>) -> Option<NodeId> {
    match input[1] {
        "ls" => None,
        "cd" => match input[2] {
            ".." => Some(get_parent(tree, node)),
            _ => Some(add_dir(tree, node, input)),
        },
        _ => panic!(),
    }
}

fn get_parent(tree: &mut Tree<FsItem>, node: &NodeId) -> NodeId {
    let node = tree.get(node).unwrap();
    node.parent().unwrap().clone()
}

fn add_dir(tree: &mut Tree<FsItem>, node: &NodeId, input: Vec<&str>) -> NodeId {
    let dir = FsItem {
        fstype: FsType::Dir,
        size: 0,
        name: input[2].to_string(),
    };
    tree.insert(Node::new(dir), UnderNode(&node)).unwrap()
}

fn add_file(tree: &mut Tree<FsItem>, node: &NodeId, input: Vec<&str>) -> Option<NodeId> {
    if let Ok(x) = input[0].parse::<usize>() {
        let file = FsItem {
            fstype: FsType::File,
            size: x,
            name: input[1].to_string(),
        };
        tree.insert(Node::new(file), UnderNode(&node)).unwrap();
    } else {
        panic!()
    }

    None
}
