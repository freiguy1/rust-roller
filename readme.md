### Tabletop RPG dice rolling application.

This is an application written to serve a useful purpose by itself but also to learn the language of [rust](http://rust-lang.org).

For the shell UI, I'm using [rustbox](https://github.com/gchp/rustbox).

Simply clone/branch the repository, navigate to the root directory and run `cargo run`.  You should be presented with a console-based UI similar to this (minus fancy transparency):

![Screenshot](https://lh3.googleusercontent.com/Oxt4XoW6wICP3A8Vv03bLrRkt_7d8KUsYznN1ysqkbc=w998-h666-no)

-------

#### Instructions

These are the instructions for what I plan on implementing:

- Use the `up` and `down` buttons to move between input
- `Tab` works the same as `down`
- `S` will save the current input; it's added to the saved list
- `Enter` will perform a roll using the textbox dice data
- `l` will load the currently selected save or history item
- `C` will clear all data in the application
- `c` will clear all data in the left text boxes
- `q` will quit out of the application


-------

#### License

MIT
