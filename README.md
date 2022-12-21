# `humansort`

*A tool for sorting items by subjective criteria*

I have a lot of ideas for future projects, but I also have trouble deciding
which one to work on next. I created `humansort` to help me find out which of my
ideas actually hold my interest. But `humansort` isn't just for lists of ideas —
you could use it for anything with a subjective comparator function, like baby
names or books.

`humansort` uses the [Elo rating system][elo], which Arpad Elo invented for
chess, but which turns out to have lots of applications in competitive games. In
a sense, `humansort` may be viewed as a "competition" among little bits of text.
The winners float to the top.

[elo]: https://en.wikipedia.org/wiki/Elo_rating_system

## CLI

1. Create a file with items to sort, one on each line.
2. Convert it to a humansort file with `cargo run -p humansort-cli -- new <name
   of file>`.
3. Sort interactively with `cargo run -p humansort-cli -- sort <name of
   file>.humansort`. During each iteration, press the number key associated with
   the item you rank highest by your subjective criteria (e.g., highest
   preference).
4. After many iterations, exit with <kbd>Ctrl</kbd> + <kbd>C</kbd>.
5. Print the sorted list in descending order with `cargo run -p humansort-cli --
   output <name of file>.humansort`.
6. If you add items to the original list later and want to sort them, add them
   to `humansort` with `cargo run -p humansort-cli -- merge <name of file> <name
   of file>.humansort`.

## Web

TODO
