### shelf
`shelf` is a command line tool used to bookmark and access files. 

#### Installation and usage
To install `shelf`, run 
```
cargo install shelf
```

To use `shelf`, the help message is already straightforward.
```
A tool to bookmark files

Usage: shelf [OPTIONS] <COMMAND>

Commands:
  remove  Remove a previously added mark
  open    Open a previously added mark
  add     Add a new mark with <id> to <file>
  list    List all recorded marks
  help    Print this message or the help of the given subcommand(s)

Options:
      --dir <DIR>  The directory to store the file map in [default: /home/ecmm/.config/shelf]
  -h, --help       Print help
  -V, --version    Print version
```
