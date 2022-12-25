# MUC
Visualize your most used commands

### Usage

Installing
```sh 
git clone https://github.com/nate-sys/muc.git
cd muc
cargo build -r && cargo install --path .
```

Running
```sh
muc --file $HISTFILE --count 10 # Bash or Zsh
muc --file $HISTFILE --count 10 --prefix="- cmd: " # Fish
```
