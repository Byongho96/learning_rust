# Develop Cargo Project
* Check the rust code  
  ```bash
  cargo check # check the rust code
  ```

* Run the project  
  ```bash
  cargo build # compile the project

  ./target/debug/hello_cargo # run the project
  ```
  or
  ```bash
  cargo run # compile and run the project
  ```

# Build for release
Compile the project with optimizations. This will generate a binary in `./target/release/` directory.

```bash
cargo build --release
```

# Library Crate
Which contains code that is intended to be used in other programs and can’t be executed on its own.
[Crates.io](https://crates.io/) is where people in the Rust ecosystem post their open source Rust projects for others to use.

```bash
cargo build
```

## Cargo.lock
- Cargo.lock file keeps track of the exact versions of dependencies that you’ve used.
- It’s automatically generated and updated by Cargo when you build your project for the first time.
- It’s used to ensure that everyone who builds your project gets the same dependency versions as you did.
- It can be updated when you update your dependencies explicitly using `cargo update`.

## Updating Dependencies
Ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml.

```bash
cargo update
```

## Dead code elimination
- Rust compiler will remove dead code from the final binary.\
- Therefore, the expression you use the library crate doesn't increase the size of the final binary.