# MUC
Visualize your most used commands  

![2022-12-25-19:50:46-screenshot](https://user-images.githubusercontent.com/96471299/209476133-d66301a3-8a5c-4c2d-9a09-ee67211f3b8f.png)

### Usage

Installing
```sh 
cargo install --git=https://github.com/nate-sys/muc
```

Running
```sh
muc --file $HISTFILE --count 10 --pretty                    # Bash / Zsh
muc --file $HISTFILE --count 10 --prefix="- cmd: " --pretty # Fish
```

### TODO
- [X] Colors
- [ ] Customizable bar
- [ ] More command line options
- [ ] Make it not crash when it finds emojis
