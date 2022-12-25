# MUC
Visualize your most used commands  

![2022-12-25-19:50:46-screenshot](https://user-images.githubusercontent.com/96471299/209476133-d66301a3-8a5c-4c2d-9a09-ee67211f3b8f.png)

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

### TODO
- [ ] Colors
- [ ] Customizable bar
- [ ] More command line options
- [ ] Make it not crash when it finds emojis
