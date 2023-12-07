# Summary

A simple CLI app to get the nth fibonacci number.

## Requirements

To build this application you need to have Rust installed in your machine, if you still don't have it head to the rust website by clicking [here](https://www.rust-lang.org/learn/get-started).

## How to use

Once you're done, you can clone this repository and open the folder with your terminal and run the following command `cargo build --release`. With that, cargo will create a binary executable and you can just run the program.

### Steps

Building the project

```bash
  $ cargo build --release

    Compiling nth-fibonacci v0.1.0 (/home/user/folder/nth-fibonacci)
      Finished release [optimized] target(s) in X s
```

This command will create a binary in the `/target/release` folder with the name of the project `nth-fibonacci`. You can run that program by doing the following

```base
  $ ./target/release/nth-fibonacci
```

If for some reason it doesn't work you might need to give it permissions by running the `chmod` command
