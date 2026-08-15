# Downloads

You can install the ZBR CLI using your preferred package manager or by downloading the binary directly. 

Further information is also available on the [ZBR documentation website](https://zbrlang.tech/docs/guides/installation).

## Recommended Installation Methods

#### Option 1: NPM
The standard Node.js package manager. You must have [Node.js](https://nodejs.org/) installed.

```bash
npm install -g @zbrlang/zbr
```

#### Option 2: Bun
A fast, all-in-one JavaScript runtime and package manager. You must have [Bun](https://bun.sh/) installed.

```bash
bun install -g @zbrlang/zbr
```

---

## Manual Download

If you prefer to download the binary directly, you can fetch it from our package repository.

> **Note:** Replace `[packageName]` with the appropriate package for your system from the list below.

| Platform | Package Name |
| :--- | :--- |
| **Linux (x64)** | `zbr-linux-x64` |
| **Linux (arm64)** | `zbr-linux-arm64` |
| **macOS (x64)** | `zbr-darwin-x64` |
| **macOS (arm64)** | `zbr-darwin-arm64` |
| **Windows (x64)** | `zbr-windows-x64` |
| **Windows (arm64)** | `zbr-windows-arm64` |

### Direct Download Links

You can access the binaries using the following pattern:

`https://packages.zbrlang.tech/[packageName]`

For example, to download the Linux x64 binary, use:
[https://packages.zbrlang.tech/zbr-linux-x64](https://packages.zbrlang.tech/zbr-linux-x64)

## How to check your architecture
If you are unsure which package to download, run the following command in your terminal:

*   **macOS / Linux:**
    ```bash
    uname -m
    ```
    *(Output `x86_64` means x64, `arm64` or `aarch64` means arm64)*

*   **Windows (PowerShell):**
    ```powershell
    (Get-CimInstance Win32_Processor).Architecture
    ```
    *(Output `9` means x64, `12` means arm64)*
