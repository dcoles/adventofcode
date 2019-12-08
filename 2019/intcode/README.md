# Intcode Emulator

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
