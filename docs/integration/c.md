# C integration

This guide shows how to call UniWorld from C using the C FFI API.

## 1. Build the C-capable library

From the repo root:

```bash
cargo build --release --features cffi
```

This produces a library (for example, `libuniworld.a` or `uniworld.dll`) under `target/release/`.

## 2. Generate a header (optional)

You can use `cbindgen` to generate a header from `src/c_bindings.rs`:

```bash
cbindgen --crate uniworld --output uniworld.h
```

Alternatively, you can declare the functions manually based on `src/c_bindings.rs`.

## 3. C API overview

The C bindings expose functions like:

- `char *uniworld_normalize_nfc(const char *text);`
- `char *uniworld_normalize_nfd(const char *text);`
- `char *uniworld_to_lowercase(const char *text);`
- `char *uniworld_to_uppercase(const char *text);`
- `unsigned int uniworld_display_width(const char *text);`
- `unsigned long long uniworld_move_right(const char *text, unsigned long long current);`
- `unsigned long long uniworld_move_left(const char *text, unsigned long long current);`
- `void uniworld_free_string(char *ptr);`

All strings are UTF-8. Returned strings must be freed with `uniworld_free_string`.

## 4. Example (C)

```c
#include "uniworld.h"
#include <stdio.h>

int main(void) {
    const char *text = "Hello, \xe4\xb8\x96\xe7\x95\x8c";

    char *nfc = uniworld_normalize_nfc(text);
    if (nfc == NULL) {
        return 1;
    }

    unsigned int w = uniworld_display_width(nfc);
    printf("NFC: %s\n", nfc);
    printf("Display width: %u\n", w);

    uniworld_free_string(nfc);
    return 0;
}
```

Link against the built UniWorld library when compiling this program.

## More information

- **[uniworld.world](https://uniworld.world)** -- Full project documentation and ecosystem
- **[GitHub repository](https://github.com/aguywithai/uniworld)** -- Source code, issues, conformance tests
- **[Other integration guides](README.md)** -- Python, JavaScript/WASM, Go
- **[VS Code extension](../../extensions/vscode/README.md)** -- UniWorld in your editor
- **[PowerShell module](../../extensions/powershell/README.md)** -- UniWorld in your terminal (also uses C FFI)
- **[Unicode Showcase](../UniWorld_Unicode_Showcase_TEST_OUTPUT.md)** -- Multi-script stress test