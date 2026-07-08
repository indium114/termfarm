# termfarm

**termfarm** is a simple CLI idle farming game written in *Rust*.

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/indium114/termfarm)

## Features

- A *rotating market* with various crop seeds to purchase, with fluctuating prices.

>[!NOTE]
>The shop rotates every **four hours**

- A **stats** subcommand showing useful stats about the current state of your farm.
    - You can even add it to your `.zshrc` or `.bashrc` to see the status of your farm whenever you open a terminal!

- The ability to **expand your farm** by purchasing new *plots* to plant crops on, using the `termfarm buy-plot` command. The price of a plot increases with each purchase

## Installation

### from the Binary

Go to the *Releases* section on the right, click the latest release, and click the binary for your architecture to download it.

> [!note]
> On macOS, you will have to compile `termfarm` from source.

### with [wares](https://github.com/indium114/wares)

Simply add the following to your `config.yaml`:

```yaml
wares:
  termfarm:
    name: termfarm
    repo: indium114/termfarm
    asset: "termfarm_Linux_x86_64"
```
> replace `x86_64` with `arm64` if you're on an ARM processor.

### with cargo

Run the following to install *termfarm*. Ensure that `~/.cargo/bin` is in your `$PATH`

```shell
cargo install termfarm
```
