# Stationeers IC10 Preprocessor

A very simple preprocessor that replaces jumps to labels with hardcoded offsets. This does not perform actual MIPS parsing, so if you try and use keywords like `yield` as a label this will horribly break your code.

I recommend pairing this with [icX](https://traineratwot.aytour.ru/wiki/icx) to strip out aliases, comments, and whitespace.

This preprocessor also uses an obscure trick to save a `yield` instruction if you have a pattern like

```
# rest of your program
yield
j some_label
```

where `some_label:` is on any line *except* line 0.

## Usage
Simply pass your files in as arguments, like so:
```
stationeers-ic10-preprocessor.exe my-program.ic10
```

In Windows, you can drag-and-drop a file onto an exe to run it in this way.

**It will overwrite your input files with the minified output, so be sure to keep a copy of the original somewhere!**

## Example

Input (7 lines):
```mips
alias accumulator r0
alias display d0
start:
add accumulator accumulator 1
move display accumulator
yield
j start
```

Output (5 lines):
```mips
alias accumulator r0
alias display d0
add accumulator accumulator 1
move display accumulator
j -2
```

## Building

Build with `cargo build --release`
