[![Actions Status](https://github.com/konradmalik/baywatch/actions/workflows/main.yml/badge.svg)](https://github.com/konradmalik/baywatch/actions)

# baywatch

Watch files and execute commands if they change.

## Usage

```bash
$ bwatch --help
```

Simplest example:

```bash
$ bwatch -- ls -lah
```

## Assumptions

-   simple codebase and usage
-   sane defaults
    -   using local gitignore properly
    -   ignores change events that happened since starting the scheduled command (not running tests 10 times if files changed
        10 times)
    -   streams stdout and stderr
-   tested and used only on Linux and Darwin, may or may not work on Windows

## Installation

```
$ cargo install --locked baywatch
```

or

Use the provided package via flake

or

```
$ nix build
```

or

```
$ nix run
```

## Similar to

-   [watchexec](https://github.com/watchexec/watchexec)
