# Tetris in the terminal
## Powered by Rust and the [ruscii engine](https://github.com/lemunozm/ruscii)

Uses the [Super Rotation System](https://tetris.wiki/Super_Rotation_System)

### Controls

X, Up = `rotate clockwise`    
Z = `rotate counterclockwise`    
Left, right = `move piece`    
Space, Down = `drop piece`    
Esc, Q = `quit`    

### Dependencies

Ruscii requires `libx11-dev` to build. Install it using your favourite package manager.

### Running

Build from source:
```
cargo run
```
or
```
cargo build
```

Or download the binary for your operating system from the releases tab.

### Wayland

Ruscii does not support Wayland. The only way that other packages have been able to get keystrokes on Wayland has been through kernel-level monitoring. If you are using a Wayland based window manager or desktop environment, running this in a terminal under XWayland will work.

### Known Bugs

- ~~Rotations of 'O' piece cause the piece to jump~~
- Next piece view missing
- Scoring formula incorrect
- Levels not implemented
- Drop speeds and timings are incorrect

### Roadmap

- [X] Fix 'O' rotations
- [ ] Fix scoring formula
- [ ] Fix drop speeds
- [ ] Implement levels
- [ ] Implement next piece view
- [ ] Add dot file for choosing which feature set is wanted
- [ ] Implement [Tetris Random Generator](https://tetris.wiki/Random_Generator), make default
- [ ] Implement hold
- [ ] Add controls options to dot file
