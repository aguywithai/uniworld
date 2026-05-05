# UniWorld Integration Guides

Per-language integration docs for UniWorld's bindings. Each guide covers building, installing, and using UniWorld from the target language.

UniWorld's core is implemented in Rust. All bindings share the same algorithms, data tables, and conformance test results. The behavior is identical across languages because it is the same code.

| Language | Guide | Package |
|----------|-------|---------|
| **Python** | [python.md](python.md) | [pypi.org/project/uniworld](https://pypi.org/project/uniworld/) |
| **JavaScript / WASM** | [javascript-wasm.md](javascript-wasm.md) | [npmjs.com/package/uniworld](https://www.npmjs.com/package/uniworld) |
| **C** | [c.md](c.md) | Build from source (`cargo build --features cffi`) |
| **Go** | [go.md](go.md) | Build from source (CGo wrapper over C FFI) |

## Developer tools

UniWorld also ships as ready-to-use developer tools:

| Tool | Link | Guide |
|------|------|-------|
| **VS Code extension** | [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=aguywithai.uniworld) | [Extension README](../../extensions/vscode/README.md) |
| **PowerShell module** | [PowerShell Gallery](https://www.powershellgallery.com/packages/UniWorld) | [Module README](../../extensions/powershell/README.md) |

## More information

- **[uniworld.world](https://uniworld.world)** -- Full documentation, install guides, and the complete UniWorld ecosystem
- **[GitHub repository](https://github.com/aguywithai/uniworld)** -- Source code, issues, and contributions
- **[Unicode Showcase](../UniWorld_Unicode_Showcase_TEST_OUTPUT.md)** -- Multi-script stress test demonstrating UniWorld across all supported scripts
- **[A Guy With AI](https://aguywithai.world)** -- Publisher: podcast and open-source development
- **[HAIMU](https://haimu.world)** -- Sean MacNutt's AI development methodology; HAIMU generated the insight leading to UniWorld and enabled 14-hour library development from idea to implementation. "Move fast and fix things."
- **[Grand Beta](https://grandbeta.world)** -- Business entity that funded development
