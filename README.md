# MUC
Visualize your most used commands  

![2022-12-26-08:24:06-screenshot](https://user-images.githubusercontent.com/96471299/209506463-8ea9f181-63a5-47ce-8279-168d3161a7a3.png)

## Usage

### Installing

#### Arch users (AUR)

Use your favorite AUR helper to install [muc-git](https://aur.archlinux.org/packages/muc-git) package (or build manually using `git` and `makepkg -si`), for example: `paru -S muc-git`

#### Other distros

```sh
cargo install --git=https://github.com/nate-sys/muc
```

### Running

```sh
muc --file $HISTFILE --count 10 --pretty                    # Bash / Zsh
muc --file $HISTFILE --count 10 --prefix="- cmd: " --pretty # Fish

muc --file $HISTFILE\
        --count 10\
        --pretty\
        --bar '*'\
        --bar-open '('\
        --bar-close ')'                                     # (******    ) 
```

### TODO
- [X] Colors
- [X] Customizable bar
- [ ] More command line options
