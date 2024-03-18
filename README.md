# Advent Of Code 2023

These are my solutions for [Advent Of Code](https://adventofcode.com) 2023, written in Rust. It's not pretty as I'm still a beginner in Rust, and usually don't take the time to refactor (it's AOC, not some code I have to maintain ;-).

Each day directory is supposed to contain a `rsc` directory, where the `input.txt` should be put.


## ARM64

To learn about ARM64 (AArch64) development, I've implemented day 1 and day 16 in pure ARM64 assembly. Although you
[cannot run a static binary on modern macOS](https://github.com/apple-oss-distributions/xnu/blob/5e3eaea39dcf651e66cb99ba7d70e32cc4a99587/bsd/kern/mach_loader.c#L852-L874)
(except on x86_64, for some reason), you don't need to link against any library/framework. Officially, running static
executables was never supported on macOS as it does not provide a public, stable kernel ABI (unlike Linux, which is
somewhat an exception in this respect).

But you can do kernel calls manually; it just involves reading some kernel header files.
See [syscall.inc](ARM64/lib/syscall.inc) for some examples. BSD kernel calls have positive syscall numbers, Mach
kernel calls are negative.

And so I've implemented [simple memory allocation](ARM64/lib/malloc.s) and [querying the time](ARM64/lib/time.s)
via kernel calls. Another fun part is [accessing the COMM PAGE](ARM64/lib/mach.s) which is a memory page mapped into
every executable that provides some informations without the need to do system calls. I need it to get the memory
page size in order to correctly calculate the size of memory to request from the kernel.

Implementing [day 1](ARM64/day1/day1.s) was straight-forward as it didn't even need dynamic memory allocation.
Most of the work was writing utilities to print strings and numbers, and iterating the input string.

For a bigger challenge, I then implemented [day 16](ARM64/day16/day16.s) and that involved a lot more utilities. That
implementation is mostly a port of my [Rust version of day 16](day16/src/main.rs) and most functions should follow the
ARM/Apple ABI. This means most functions could be replaced by functions written in C, except for `next_pos` which
returns values in registers X0, X1, and X2 (which is not how this should be done according to the
[Procedure Call Standard for the Arm® 64-bit Architecture (AArch64)](https://github.com/ARM-software/abi-aa/releases)).

I then tried to [optimize day 16 even further](ARM64/day16a/day16a.s), partially by deliberately violating the ABI.
For example, in several functions, that implementation stores/passes data via registers that would usually be
call-clobbered and thus would need to be saved on the stack.

On my MacBook Pro 2021 with M1 Max, I get roughly these timings for day 16 part 2:

| Implementation         | Runtime |
| -----------------------|---------|
| Rust (release build)   | 260ms   | 
| Assembler, unoptimized | 75ms    |
| Assembler, optimized   | 45ms    |

One can likely further improve my assembler version and the Rust implementation (which I didn't even try to further
optimize). It's Good Enough™ for me, though.
