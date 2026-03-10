- Stores all artifacts (scripts, outputs, basis dumps, stats)
- Optionally compares the resulting bases

------------------------------------------------------------------------

# Requirements

## 1️⃣ Rust

You need a recent stable Rust toolchain.

``` bash
rustc --version
cargo --version
```

If not installed:

``` bash
curl https://sh.rustup.rs -sSf | sh
```

------------------------------------------------------------------------

## 2️⃣ Singular (optional but recommended)

To run the `singular` backend you must install Singular.

### Debian / Ubuntu

``` bash
sudo apt update
sudo apt install singular
```

### Arch

``` bash
sudo pacman -S singular
```

### Fedora

``` bash
sudo dnf install singular
```

Verify installation:

``` bash
Singular --version
```

If Singular is not in your `PATH`, you can pass:

``` bash
--singular.bin /full/path/to/Singular
```

------------------------------------------------------------------------

# Build

From workspace root:

``` bash
cargo build -p test-engine
```

Or inside `engine/`:

``` bash
cargo build
```

------------------------------------------------------------------------

# Test Case Format

Example `grobner_cases.toml`:

``` toml
[[case]]
name = "mod_p_example"
field = "Fp"
p = 32003
vars = ["x", "y"]
order = "dp"
generators = [
  "x^2 + y^2 - 1",
  "x^3 - y"
]
```

------------------------------------------------------------------------

# Run

## Run both backends

``` bash
cargo run -p test-engine -- --backend both
```

## Run both + compare results

``` bash
cargo run -p test-engine -- --backend both --compare
```

## Run only GBX

``` bash
cargo run -p test-engine -- --backend gbx
```

## Run only Singular

``` bash
cargo run -p test-engine -- --backend singular
```

## Clean output before running

``` bash
cargo run -p test-engine -- --clean --compare
```

------------------------------------------------------------------------

# Output Structure

After running, you will see:

    target/
      scripts/
        singular/
        gbx/
      outputs/
        singular/
        gbx/
      bases/
        singular/
        gbx/
      stats/
        singular/
        gbx/
      compare/

------------------------------------------------------------------------

# Philosophy

This test engine exists to:

- Guarantee correctness of GBX
- Detect regression early
- Compare against trusted CAS
- Provide reproducible benchmarking artifacts
