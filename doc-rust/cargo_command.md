# CARGO COMMANDS

### Create program:

```
  cargo new "myprogram"
  cargo new --vcs=none "myprogram" (disable git)
```

### Create lib:

```
  cargo new --lib "mylib"
```

### Compile program:

```
  cargo build (you can also compile with rustc "myprogram")
```

### Check if code compile:

```
  cargo check (more faster than build each time when you write code)
```

### Compile and optimise for production code:

```
  cargo build --release (bin generate in target/release instead of target/debug)
```

### Update crates: (by default cargo doesnt update package to maintain the app)

```
  cargo update
```

### Install globaly:

```
  cargo install "module"
```

### (Install cargo-edit provide a way to manage module via CLI)

```
  cargo-add "module"
  cargo-remove "module"
  cargo-setversion...
```

### Install cargo-modules provide a way to visualize modules inside your project)

```
    cargo modules generate tree (Visualize modules)
    cargo modules generate tree --with-types (Visualize modules, Data and Privacy)
```

### Cargo Watch (Watch change on a cargo project)

```
    OPTIONS:
    -q quietly, -c clear clear console between each watch, -w watch, -x execute

    EXAMPLES:
    cargo install cargo-watch (Install globaly cargo watch)
    cargo watch -q -c -w src -x run (Watch src 
    cargo watch -q -c -w tests -x "test -q my_test_function -- --nocapture"
```
