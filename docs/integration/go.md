# Go integration

This guide shows how to use UniWorld from Go via the CGo wrapper.

## 1. Build the UniWorld C library

From the repo root:

```bash
cargo build --release --features cffi
```

This produces a C-capable library in `target/release/` (e.g. `libuniworld.a`, `uniworld.dll`, or `libuniworld.so` depending on platform).

## 2. Configure Go to link against UniWorld

In your Go environment, set `CGO_LDFLAGS` so that the linker can find UniWorld, for example:

```bash
export CGO_LDFLAGS="-L/path/to/target/release -luniworld"
```

On Windows with `uniworld.dll`, you may place the DLL on `PATH` instead.

## 3. Import the Go wrapper

The Go wrapper lives under `bindings/go/uniworld.go`. You can either:

- Use it as a module in-place, or
- Copy it into your own project under an appropriate module path.

Example module usage:

```go
package main

import (
    "fmt"

    uw "example.com/uniworld/bindings/go"
)

func main() {
    text := "Hello, \u4e16\u754c \U0001f44b"

    nfc := uw.NormalizeNFC(text)
    fmt.Println("NFC:", nfc)

    width := uw.DisplayWidth(nfc)
    fmt.Println("Display width:", width)

    lower := uw.ToLowercase("Stra\u00dfe")
    fmt.Println("Lowercase:", lower)
}
```

## 4. Available functions

The Go wrapper currently exposes:

- `NormalizeNFC(text string) string`
- `NormalizeNFD(text string) string`
- `ToLowercase(text string) string`
- `ToUppercase(text string) string`
- `DisplayWidth(text string) uint32`
- `MoveRight(text string, current uint64) uint64`
- `MoveLeft(text string, current uint64) uint64`

These map directly onto the C FFI functions in `src/c_bindings.rs`.

## More information

- **[uniworld.world](https://uniworld.world)** -- Full project documentation and ecosystem
- **[GitHub repository](https://github.com/aguywithai/uniworld)** -- Source code, issues, conformance tests
- **[Other integration guides](README.md)** -- Python, JavaScript/WASM, C
- **[VS Code extension](../../extensions/vscode/README.md)** -- UniWorld in your editor
- **[PowerShell module](../../extensions/powershell/README.md)** -- UniWorld in your terminal
- **[Unicode Showcase](../UniWorld_Unicode_Showcase_TEST_OUTPUT.md)** -- Multi-script stress test