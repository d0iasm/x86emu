# x86emu
x86 emulator in Rust. This is a porting from the implementation written by C in "自作エミュレータで学ぶx86アーキテクチャ". You can see the original code in the [support page](https://book.mynavi.jp/support/bookmook/x86/).

It can now execute a binary file to calculate fibonacci numbers. There is a binary file for calculating a fibonacci number 10 (=55) in the test directory. The progress corresponds to tolset_p86/emu3.10 which is the part of the original source code.

## Usage
```
$ cargo run test/mov_r32_imm32
```

## References
- https://book.mynavi.jp/support/bookmook/x86/
- https://www.felixcloutier.com/x86/
