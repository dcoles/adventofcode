# Intcode Interpreter

## Usage

```
USAGE: intcode [-A | --ascii] [-d | --debug] [-B | --break] PROGRAM
Run Intcode PROGRAM in the interpreter.

-A, --ascii    use ASCII input/output
-d, --debug    enable debugging mode (traces execution and break into debugger on exceptions)
-B, --break    immediately break into debugger
```

The interpreter reads input from stdin and prints output to stdout.
Interpreters can be linked together by piping the output of one interpreter into another.

```shell
# Run interpreter with fixed input
$ intcode $PROGRAM <<< 5

# Chain interpreters together
$ intcode $PROGRAM | intcode $PROGRAM
```

The easiest way of running from a git checkout is using `cargo run -q --`.

## Debugger

If debugging is enabled (`--debug`) then exceptions will cause the interpreter to drop to a debugger.

```
$ intcode --debug $PROGRAM
00000000 ADD 0x00000000 0x00000000 0x00000000
ERROR: Illegal instruction 0 (ip: 0x00000004)
00000004 ???
debug>
```

The list of supported commands can be printed using `help`:

```
debug> help
p|print [ ADDR | $ip | $rb ]
                Print contents of address
c|continue      Continue execution
q|quit          Exit debugger and terminate program
d|disassemble   Disassemble current instruction
s|step          Step to the next instruction
i|input         Write input to the CPU
D|dump          Dump memory to console
h|help          Print this help
```

## Using with `binfmt_misc`

You can register this binary as the handler of `.intcode` files on Linux by
using `binfmt_misc` support in the kernel:

```shell
echo ':Intcode:E::intcode::/usr/local/bin/intcode:' > /proc/sys/fs/binfmt_mic/register
```

## Day 7

An example of how this can be used to solve
[Day 7](https://adventofcode.com/2019/day/7) by running this emulator in a
series of shell pipelines.
