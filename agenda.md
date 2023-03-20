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