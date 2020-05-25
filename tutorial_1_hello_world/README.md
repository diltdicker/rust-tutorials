# Tutorial 1: Hello World

## Install [Rust](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

after this point you will need to upgrade your installation of rust

```bash
rustup update
```

## hello_world.rs

create a file called hello_world.rs with the following

```rust

fn main() {
    println!("hello world")
}

```

Compile the file to an executable with the following

```bash
rustc -o run.exe hello_world.rs
```

There's no real need to have the extension for *nix systems but it makes it easier for .gitignore files

Now run the program with the following

```bash
./run.exe
```
