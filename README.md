# MUC
Visualize your most used commands  

![2022-12-26-08:24:06-screenshot](https://user-images.githubusercontent.com/96471299/209506463-8ea9f181-63a5-47ce-8279-168d3161a7a3.png)

### Usage

Installing
```sh 
cargo install --git=https://github.com/nate-sys/muc
```

Running
```sh
muc --file $HISTFILE --count 10 # Bash or Zsh
muc --file $HISTFILE --count 10 --prefix="- cmd: " # Fish
```

### TODO
- [X] Colors
- [ ] Customizable bar
- [ ] More command line options
- [ ] Make it not crash when it finds emojis
