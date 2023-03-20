# Add devcontainer file
```JSON
{
  "image": "mcr.microsoft.com/devcontainers/universal:2",
  "portsAttributes": {
    "3000": {
      "label": "Application",
      "onAutoForward": "openPreview"
    }
  },
  "forwardPorts": [3000]
}
```


# Add dotfiles
```bash
apt install sudo
git clone https://github.com/tijoer/dotfiles.git
cd dotfiles
git checkout new
chmod +x setupNewDevcontainer.sh 
sudo ./setupNewDevcontainer.sh
```

# Add Github CoPilot Plugin and right click add to devcontainer

- 

# Create a html file

```html
<!DOCTYPE html>
<html>
<head>
    <title>Example 1</title>
</head>
<body>
    <h1>Example 1</h1>
    <p>This is an example of a simple HTML document.</p>
</html>
```

# Show Copilot keybindings

Settings->Keybindings

Search for 	```editor.action.inlineSuggest```

```
alt+´
alt+ß
alt+^
ctrl+enter
alt+rightArrow
Escape
Tab
```

# create a buildAndRun.sh file

```bash
#!/bin/bash

# end on error
set -e

function ctrl_c() {
    # uft-8 red x
    echo -e "\e[31m\u2717\e[0m Aborting..."
    docker stop myapp
    docker rm myapp
    exit 1
}

# trap ctrl_c and call ctrl_c()
trap ctrl_c INT

# build the docker image
docker build -t myapp .

# run the docker image
docker run -d --name myapp -p 8080:80 myapp
```

Show Docker images `dkil`

# Update html file
```html
    <!-- Include skeleton.css from CDN -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/skeleton/2.0.4/skeleton.min.css" />
```
```html
    <!-- Include tweets by elon musk -->
    <a class="twitter-timeline" href="https://twitter.com/elonmusk?ref_src=twsrc%5Etfw">Tweets by elonmusk</a>
    <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
```
# update run script

```bash
#!/bin/bash

# end on error
set -e

# trap ctrl_c and call ctrl_c()
function ctrl_c() {
    # uft-8 red x
    echo -e "\e[31m\u2717\e[0m Aborting..."
    docker stop myapp
    docker rm myapp
    exit 1
}
trap ctrl_c INT

# kill all running containers (if any)
docker kill $(docker ps -q) > /dev/null 2>&1 || true

# remove all containers (if any)
docker rm $(docker ps -a -q) > /dev/null 2>&1 || true

# build the docker image
docker build -t myapp .

# run the docker image
docker run -d --name myapp -p 8080:80 myapp
```

# Add Rust feature to Devcontainer
```JSON
	"features": {
		"rust": {
		"description": "Rust",
		"containerImage": "mcr.microsoft.com/vscode/devcontainers/rust:1",
		"extensions": [
			"rust-lang.rust"
		]
		}
	},
```

Rebuild Devcontainer

# Add a new Rust Project
```cargo new traveling_salesman```

```Rust

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


```