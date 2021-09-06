# WAC - **W**icked **A**ddress **C**alculator

# **WARNING: DO NOT USE THIS**
**This project was created for the [Terminal Code Jam](https://togglebit.io/posts/terminal-game-jam/) hosted by [Togglebit](https://www.twitch.tv/togglebit) and contains intentional bugs that will make your life hell if you try to use this for actual osdev.**

## Description

WAC is an address calculator which can be used to calculate offsets and do bit manipulations. It only supports one number type (unsigned int64) and only shows output in hex.

## How does it fit the theme: "Pain and Suffering"?

Well, built into the calculator are certain bugs which will give you slight (and sometimes not so slight) misscalculations.
So if you would actually use it for something like osdev to calculate offsets, things will mostly work out, until they don't.

Which is where the real pain begins, because if you do the calculation a second time, it will most probably spit out the right value, making you believe you typed the wrong values the last time.
(Probably works better without interactive mode, but it's more fun to play with it that way).

### Instructions for the Jam

Open an interactive session and just play around with the caculations.
```sh
SUCCESS_RATE=0.5 cargo run -q
```

### Success Rate
The success rate can be controlled through an environment variable at compile time. This variable should be a float between 0 and 1. The default is 0.98.

Linux:
```sh
SUCCESS_RATE=0.1 cargo run -q -- "1 + 2 * 3"
```

Windows:
```bat
set SUCCESS_RATE=0.1
cargo run -q -- "1 + 2 * 3"
```

## Usage
WAC can evalute expressions passed as arguments:
```sh
cargo run -q -- "1 + 2 * 3"
cargo run -q -- "(1 + 2) * 3"
cargo run -q -- "1 GB + 0x400"
```

But can also run in interactive mode:
```sh
cargo run -q
```

To exit interactive mode you can press Ctrl-C or use one of the following commands:
```
exit
quit
halt
stop
:q
:cq
```

To show how the tool will set precedence, you can use the `show` command in front of an expression.
```sh
cargo run -q -- "show 1 + 2 * 3"
```

## Features
### Commands
| Command |         Description          |
| ------- | ---------------------------- |
| ?       | Shows help for the operators |
| help    | Shows help for the operators |
| show    | Prints the parsed expression |

### Binary Operators
| Operator |   Description    |
| -------- | ---------------- |
| +        | Addition         |
| -        | Subtraction      |
| *        | Multiplication   |
| ^        | Power            |
| /        | Integer Division |
| %        | Modulo           |
| <<       | Bit Shift Left   |
| >>       | Bit Shift Right  |
| \        | Bitwise OR       |
| &        | Bitwise AND      |
| xor      | Bitwise XOR      |

### Prefix Operators
| Operator | Description |
| -------- | ----------- |
| ~        | Bitwise NOT |

### Postfix Operators
| Operator | Description |
| -------- | ----------- |
| ++       | Increment   |
| --       | Decrement   |
| !        | Factorial   |

### Units (Case Sensitive)
| Unit | Name | Factor |
| ---- | ---- | ------ |
| b    | bit  | 1      |
| B    | byte | 8      |

### Unit Prefixes (Not Case Sensitive)
**These can only be used together with one of the units. E.G.: 1 Gb, 1 GB**
| Unit Prefix | Name  |  Factor  |
| ----------- | ----- | -------- |
| k           | kilo  | 1000     |
| ki          | kibi  | 1024     |
| m           | mega  | 1000 ^ 2 |
| mi          | mebi  | 1024 ^ 2 |
| g           | giga  | 1000 ^ 3 |
| gi          | gibi  | 1024 ^ 3 |
| t           | tera  | 1000 ^ 4 |
| ti          | tebi  | 1024 ^ 4 |
| e           | exa   | 1000 ^ 5 |
| ei          | exbi  | 1024 ^ 5 |
| p           | peta  | 1000 ^ 6 |
| pi          | pebi  | 1024 ^ 6 |
| z           | zetta | 1000 ^ 7 |
| zi          | zebi  | 1024 ^ 7 |
| y           | yotta | 1000 ^ 8 |
| yi          | yobi  | 1024 ^ 8 |

### Constants
| Constant |       Value        |         Description         |
| -------- | ------------------ | --------------------------- |
| min      | 0x0                | The lowest possible number  |
| max      | 0xffffffffffffffff | The highest possible number |
| c        | 0x11de784a         | The speed of light          |
