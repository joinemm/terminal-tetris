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
