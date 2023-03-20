
// A Node structure for a binary tree
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// print the tree
fn print_tree(node: &Node) {
    if let Some(ref left) = node.left {
        print_tree(left);
    }
    println!("{}", node.value);
    if let Some(ref right) = node.right {
        print_tree(right);
    }
}

// build the tree
fn build_tree(v: &Vec<i32>) -> Node {
    let mut root = Node {
        value: v[0],
        left: None,
        right: None,
    };
    for i in 1..v.len() {
        let mut node = &mut root;
        loop {
            if v[i] < node.value {
                if let Some(ref mut left) = node.left {
                    node = left;
                } else {
                    node.left = Some(Box::new(Node {
                        value: v[i],
                        left: None,
                        right: None,
                    }));
                    break;
                }
            } else {
                if let Some(ref mut right) = node.right {
                    node = right;
                } else {
                    node.right = Some(Box::new(Node {
                        value: v[i],
                        left: None,
                        right: None,
                    }));
                    break;
                }
            }
        }
    }
    root
}


// inverts the binary tree
fn invert_tree(node: &mut Node) {
    let temp = node.left.take();
    node.left = node.right.take();
    node.right = temp;
    if let Some(ref mut left) = node.left {
        invert_tree(left);
    }
    if let Some(ref mut right) = node.right {
        invert_tree(right);
    }
}

fn sortTree(node: &mut Node, v: &mut Vec<i32>) {
    if let Some(ref mut left) = node.left {
        sortTree(left, v);
    }
    v.push(node.value);
    if let Some(ref mut right) = node.right {
        sortTree(right, v);
    }
}

fn main() {
    // A vector of 10 random nunmbers
    let mut v = vec![0; 10];
    for i in 0..10 {
        v[i] = rand::random::<i32>();
    }

    let foo = build_tree(&v);
    let mut foo = foo;
    sortTree(&mut foo, &mut v);

    // print the tree
    print_tree(&foo);

    println!();
    println!();
    println!();
    println!();
    

    // invert the tree
    let mut foo = foo;
    invert_tree(&mut foo);


    // print the tree
    print_tree(&foo);


    print!("Hello, world!");
}
