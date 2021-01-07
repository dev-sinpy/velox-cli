## Velox-CLI
> Please ignore this repo for now. It is still under heavy development and most of the stuff doesn't work. Contact [me](mailto::dev.sinpy@protonmail.com) if you have any other questions.

A command line interface for velox framework to help developers easily create, run and build there velox apps.


## Installation
### Using Cargo
```
$ cargo install velox-cli
```
This will install velox-cli globally in your machine. Then to check if velox was successfully installed run `velox --version` in your terminal and this should display a version number.

### Build from source
> Todo


## Usage
```
USAGE:
    velox [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build    build a binary of your project
    help     Prints this message or the help of the given subcommand(s)
    init     Initialise a velox project in current directory
    new      Create a new velox project
    run      Run a binary of your project
 ```