### Tabletop RPG dice rolling application.

This is an application written to serve a useful purpose by itself but also to learn the language of [rust](http://rust-lang.org).

For the shell UI, I'm using [rustbox](https://github.com/gchp/rustbox).

Simply clone/branch the repository, navigate to the root directory and run `cargo run`.  You should be presented with a console-based UI similar to this (minus fancy transparency):

![Screenshot](https://lh4.googleusercontent.com/-x8u_2_2ARrc/VMfILOu_9nI/AAAAAAAAI5U/dKq0Tx5ZU8A/w705-h376-no/Untitled.png)

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
