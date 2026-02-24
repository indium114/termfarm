# termfarm

**termfarm** is a simple CLI idle farming game written in *Swift*.

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/StikyPiston/termfarm)

## Features

- A *rotating market* with various crop seeds to purchase, with fluctuating prices.

>[!NOTE]
>The shop rotates every **four hours**

- A **stats** subcommand showing useful stats about the current state of your farm.
    - You can even add it to your `.zshrc` or `.bashrc` to see the status of your farm whenever you open a terminal!

- The ability to **expand your farm** by purchasing new *plots* to plant crops on, using the `termfarm buyplot` command. The price of a plot increases with each purchase

## Installation

You can easily install **termfarm** through *Homebrew* on **Linux** and **macOS**. Just run the command below.

```zsh
brew install stikypiston/formulae/termfarm
```

Then, run the following command to create a **save file** with some coins that you'll need in order to buy seeds:

```zsh
termfarm init
```
