# Python integration (uniworld)

This guide shows how to use UniWorld from Python via the PyO3 bindings.

## 1. Build and install the wheel

Requirements:
- Rust toolchain (`rustup`, `cargo`)
- Python 3.8+
- `maturin` (`pip install maturin`)

From the repo root:

```bash
pip install maturin
maturin build --release --features python
```

This produces a wheel under `target/wheels/`. Install it into your environment:

```bash
pip install target/wheels/uniworld-*.whl
```

## 2. Basic usage

```python
import uniworld as uw

text = "Hello, \u0627\u0644\u0639\u0627\u0644\u0645 \U0001f44b"

# Grapheme cluster boundaries (byte offsets)
g_breaks = uw.grapheme_boundaries(text)

# Word boundaries
w_breaks = uw.word_boundaries(text)

# Sentence boundaries
s_breaks = uw.sentence_boundaries(text)

# Normalization
nfc  = uw.normalize_nfc(text)
nfd  = uw.normalize_nfd(text)
nfkc = uw.normalize_nfkc(text)
nfkd = uw.normalize_nfkd(text)

# Case mapping
lower = uw.to_lowercase(text)
upper = uw.to_uppercase(text)
title = uw.to_titlecase("the quick brown fox")
fold  = uw.case_fold("Stra\u00dfe")
```

## 3. Line breaking

```python
# Thai text with no spaces -- UniWorld uses dictionary segmentation
line = "Thai text example for UniWorld line breaking"

# (offset, action) tuples where action is "mandatory" or "allowed"
breaks = uw.line_break_opportunities_with_dictionary(line)

for offset, action in breaks:
    print(offset, action, repr(line[:offset]))
```

## 4. Display width and truncation

```python
from uniworld import display_width, truncate_graphemes, truncate_display_width

s = "Hello \u4e16\u754c \U0001f44b"
width = display_width(s)

short_g = truncate_graphemes(s, max_graphemes=5)
short_w = truncate_display_width(s, max_width=10)
```

## 5. Cursor navigation

```python
from uniworld import (
    move_right, move_left,
    move_right_visual, move_left_visual,
    select_word,
)

text = "Hello \u05e9\u05dc\u05d5\u05dd"
pos = 0

# Logical cursor movement
pos = move_right(text, pos)
pos = move_left(text, pos)

# Visual-order movement (BiDi-aware)
pos = move_right_visual(text, pos)
pos = move_left_visual(text, pos)

# Word selection
start, end = select_word("Hello world", 7)
word = "Hello world"[start:end]
```

Refer to the Python module's `help(uniworld)` output for the full list of functions.

## More information

- **[uniworld.world](https://uniworld.world)** -- Full project documentation and ecosystem
- **[UniWorld on PyPI](https://pypi.org/project/uniworld/)** -- Package page
- **[GitHub repository](https://github.com/aguywithai/uniworld)** -- Source code, issues, conformance tests
- **[Other integration guides](README.md)** -- JavaScript/WASM, C, Go
- **[VS Code extension](../../extensions/vscode/README.md)** -- UniWorld in your editor
- **[PowerShell module](../../extensions/powershell/README.md)** -- UniWorld in your terminal
- **[Unicode Showcase](../UniWorld_Unicode_Showcase_TEST_OUTPUT.md)** -- Multi-script stress test