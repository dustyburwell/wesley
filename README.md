Wesley watches for file changes and runs a command of your choosing whenever
one is encountered. Wesley is intended to be cross platform and should run
equally well on OSx, Linux, and Windows.

## Install

Wesley is built in Rust and is presently most easily installed with
[Cargo](https://github.com/rust-lang/cargo) which comes packaged with
[Rust](https://doc.rust-lang.org/book/getting-started.html#installing-rust).
With Cargo installed, run `cargo install wesley`. Ensure that `~/.cargo/bin`
is on your path and you're good to go.

## Usage

```
Usage: wesley [options] COMMAND

Options:
    -o, --only PATH     only run COMMAND when path that changed matches PATH
    -e, --exclude PATH  run COMMAND when path doesn't match PATH glob
    -h, --help          print this help menu
```

Wesley is quite useful when paired with a test runner or build tool. For
instance, the following will execute `npm test` whenever a change is detected
in the current directory, automatically executing the `test` script configured
for a Node.js project.

```
wesley npm test
```

Similarly, the following will rebuild the artifact produced for a Java project.
Note the excluded "target/*" path since the compile step will result in changes
to files in the `target` directory.

```
wesley -e "target/*" mvn compile
```

You could even use Wesley to watch changes and rebuild itself.

```
wesley -e "target/*" cargo build
```

Wesley doesn't care what command you want to run or what language or platform
you might be working with. No judgement here.
