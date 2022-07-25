# SimpleAnsi

## About

SimpleAnsi is a fucking simple library that allows basic coloring of terminal characters.

There are many crates for the ansi terminal that have been created and abandoned, but none of them solve the following problem.
If that is the case, I do not need complex functionality, so I built the library on a scale such that the code can be read at a glance.
https://github.com/jam1garner/owo-colors/issues/45

## How to use

```toml
# Cargo.toml

[dependencies]
simple-ansi = "0.1.0"
```

```rust
use simple_ansi::SimpleAnsi;
println!("{}", "red".red());
println!(
    "{}",
    "bold italic red on green".bold().italic().red().on_green()
);
```

## License

simple-ansi is released under the MIT License
