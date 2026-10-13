<div align="center">

<h2>zerobrew</h2>

<p align="center">
  <strong>English</strong> ·
  <a href="README.zh.md">中文</a>
</p>

[![Lint](https://github.com/zerobrewhq/zerobrew/actions/workflows/ci.yml/badge.svg)](https://github.com/zerobrewhq/zerobrew/actions/workflows/ci.yml)
[![Test](https://github.com/zerobrewhq/zerobrew/actions/workflows/test.yml/badge.svg)](https://github.com/zerobrewhq/zerobrew/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/zerobrewhq/zerobrew?display_name=tag)](https://github.com/zerobrewhq/zerobrew/releases)
[![Discord](https://img.shields.io/badge/Discord-Join-5865F2?logo=discord&logoColor=white)](https://discord.gg/ZaPYwm9zaw)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE-MIT.md)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE-APACHE.md)

<img alt="zerobrew demo" src="./assets/zb-demo-0.4.0.gif" />

<p><strong>zerobrew brings uv-style architecture to Homebrew packages on macOS and Linux.</strong></p>

</div>

## Install

```bash
curl -fsSL https://zerobrew.rs/install | bash
```

The installer updates your shell config. After it finishes, restart your terminal
or run the `source` command it prints.

Or via Homebrew:

```bash
brew install zerobrewhq/zerobrew/zerobrew
```

## Update zerobrew

If you used the standalone installer, rerun it:

```bash
curl -fsSL https://zerobrew.rs/install | bash
zb --version
```

If you installed with Homebrew:

```bash
brew update && brew upgrade zerobrew
```

If you installed from the old `lucasgelfond/zerobrew` tap, it's no longer updated. Switch to the new one:

```bash
brew uninstall zerobrew
brew untap lucasgelfond/zerobrew
brew install zerobrewhq/zerobrew/zerobrew
```

## Quick start

To opt in to a `brew` compatibility entrypoint:

```bash
zb install-alias brew
brew install jq
zb install-alias brew --remove
```

This creates `brew -> zb-brew` beside `zb`; ensure that directory is on PATH.
You can also enable it during installation:

```bash
curl -fsSL https://zerobrew.rs/install | bash -s -- --brew-compat
```

Enabling it refuses existing Homebrew or other `brew` entrypoints on PATH and
standard Homebrew locations, and never replaces an existing file. Removing it
only deletes zerobrew's own symlink. If `zb-brew` is missing, reinstall zerobrew.

The entrypoint supports `install`, `uninstall`, `list`, `info`, `outdated`,
`upgrade`, `doctor`, `bundle`, and `update`, using the arguments and output
supported by `zb`. `cleanup` runs `zb gc` and only removes unreferenced store
entries. `update` refreshes the formula index; it does not update zerobrew itself.
Commands such as `search`, `services`, and `tap`, and flags such as `--cask` and
`list --formula`, are not implemented and fail with a compatibility error.
`zb migrate` requires real Homebrew on PATH and refuses this entrypoint.

```bash
zb install jq                   # install one package
zb install wget git             # install multiple
zb bundle                       # install from Brewfile
zb bundle install -f myfile     # install from custom file
zb bundle dump                  # export installed packages to Brewfile
zb bundle dump -f out --force   # dump to custom file (overwrite)
zb uninstall jq                 # uninstall one package
zb outdated                     # list packages with newer versions
zb upgrade                      # upgrade all outdated packages
zb upgrade jq wget              # upgrade specific packages
zb reset                        # uninstall everything
zb gc                           # garbage collect unused store entries
zbx jq --version                # run without linking
```

Automatic initialization (`--auto-init` or `ZEROBREW_AUTO_INIT=true`) and `zb reset`
create data directories without changing shell configuration. Run `zb init`
explicitly to configure your shell, or `zb init --no-modify-path` to leave it unchanged.

## Performance snapshot

<div align="center">

| Package | Homebrew (cold) | ZB (cold) | Cold speedup | Homebrew (warm) | ZB (warm) | Warm speedup |
|---------|-----------------|-----------|--------------|-----------------|-----------|--------------|
| **Overall (100 packages)** | 782s | 216s | **3.6x** | 650s | 27.7s | **23x** |
| python@3.14 | 17.10s | 6.44s | 2.7x | 15.33s | 440ms | 34.8x |
| node | 28.37s | 7.24s | 3.9x | 26.80s | 743ms | 36.1x |
| tesseract | 35.42s | 5.20s | 6.8x | 33.95s | 2.88s | 11.8x |
| openjdk | 31.64s | 6.80s | 4.7x | 29.65s | 2.93s | 10.1x |
| llvm | 26.15s | 6.50s | 4.0x | 9.78s | 191ms | 51.2x |

</div>

Measured on 2026-10-12 with zerobrew 0.4.0+dev and Homebrew 7.0.9 on macOS 26.6.2, MacBook Pro (M3 Pro, 18 GB RAM), ~374 Mbit/s download bandwidth. Homebrew started with nothing installed.

- **Cold**: the package and all of its dependencies uninstalled, empty download cache.
- **Warm**: the package and all of its dependencies uninstalled, downloads from the cold run still cached.
- The median package was 4.8x faster cold and 52x faster warm. Per package, cold ranged from 1.4x (ca-certificates) to 16x (jpeg-xl) and warm from 3.5x (fontconfig) to 232x (ca-certificates). 17 of the 100 packages installed 100x faster or more warm.
- Both tools run post-install steps here. ca-certificates' step rebuilds the CA bundle from the system keychain, about 5 seconds under either tool, and nearly everything depends on it, so it sets the cold floor for most of the table. zerobrew caches the bundle, which is why ca-certificates is 27ms warm. fontconfig's `fc-cache` is the same story and is not cached yet ([#448](https://github.com/zerobrewhq/zerobrew/issues/448)), which is what holds fontconfig, harfbuzz, cairo and pango under 10x warm.
- Cold installs are bound by the link. The same run on a ~69 Mbit/s connection ([results/2026-10-07](results/2026-10-07/benchmark.md)) came out at 3.3x cold, before post-install. The run from before zerobrew ran post-install steps is [results/2026-10-08](results/2026-10-08/benchmark.md): 6.6x cold and 68x warm, with Homebrew doing that work and zerobrew not.
- Warm installs use more disk. After an uninstall brew keeps the bottle, we keep the bottle and the unpacked keg in the store. That's the whole reason a reinstall is a clone and not a rebuild. `zb gc` clears it. I'll measure both for the same set and put the numbers here.

<details>
<summary>Full results</summary>

| Package | Homebrew (cold) | ZB (cold) | Cold speedup | Homebrew (warm) | ZB (warm) | Warm speedup |
|---------|-----------------|-----------|--------------|-----------------|-----------|--------------|
| **Overall (100 packages)** | 781.58s | 215.64s | **3.62x** | 650.09s | 27.74s | **23.43x** |
| ca-certificates | 7.25s | 5.35s | 1.36x | 6.25s | 27ms | 231.67x |
| openssl@3 | 9.96s | 5.95s | 1.67x | 9.09s | 78ms | 116.55x |
| xz | 2.36s | 575ms | 4.11x | 1.40s | 27ms | 51.96x |
| sqlite | 2.98s | 588ms | 5.08x | 1.98s | 25ms | 79.24x |
| readline | 2.56s | 917ms | 2.79x | 1.43s | 29ms | 49.38x |
| icu4c@78 | 3.33s | 1.37s | 2.43x | 1.72s | 28ms | 61.57x |
| python@3.14 | 17.10s | 6.44s | 2.66x | 15.33s | 440ms | 34.84x |
| awscli | 27.04s | 8.25s | 3.28x | 25.70s | 742ms | 34.64x |
| node | 28.37s | 7.24s | 3.92x | 26.80s | 743ms | 36.06x |
| harfbuzz | 22.32s | 4.62s | 4.83x | 20.59s | 2.80s | 7.36x |
| ncurses | 2.97s | 962ms | 3.09x | 2.03s | 47ms | 43.17x |
| gh | 2.54s | 951ms | 2.67x | 1.26s | 35ms | 36.09x |
| pcre2 | 2.54s | 780ms | 3.26x | 1.49s | 33ms | 45.00x |
| libpng | 2.29s | 409ms | 5.60x | 1.43s | 26ms | 54.81x |
| zstd | 3.60s | 550ms | 6.54x | 2.68s | 32ms | 83.62x |
| glib | 6.39s | 1.43s | 4.46x | 5.15s | 93ms | 55.39x |
| lz4 | 2.28s | 394ms | 5.79x | 1.42s | 27ms | 52.41x |
| gettext | 4.26s | 1.02s | 4.16x | 3.18s | 64ms | 49.66x |
| libngtcp2 | 10.78s | 6.59s | 1.64x | 9.68s | 345ms | 28.06x |
| libnghttp3 | 2.34s | 416ms | 5.63x | 1.44s | 25ms | 57.68x |
| pkgconf | 2.46s | 449ms | 5.47x | 1.44s | 27ms | 53.19x |
| libunistring | 2.51s | 613ms | 4.10x | 1.44s | 27ms | 53.37x |
| mpdecimal | 2.36s | 460ms | 5.14x | 1.43s | 26ms | 54.92x |
| brotli | 2.53s | 536ms | 4.73x | 1.43s | 28ms | 51.04x |
| jpeg-turbo | 2.45s | 581ms | 4.22x | 1.42s | 27ms | 52.63x |
| xorgproto | 2.19s | 470ms | 4.67x | 1.21s | 29ms | 41.76x |
| ffmpeg | 18.94s | 6.73s | 2.81x | 17.81s | 373ms | 47.75x |
| cmake | 3.00s | 1.39s | 2.17x | 1.69s | 79ms | 21.43x |
| libnghttp2 | 2.36s | 443ms | 5.33x | 1.47s | 27ms | 54.30x |
| go | 5.51s | 3.14s | 1.75x | 3.28s | 192ms | 17.07x |
| uv | 2.50s | 856ms | 2.92x | 1.23s | 29ms | 42.34x |
| gmp | 2.44s | 604ms | 4.04x | 1.44s | 27ms | 53.30x |
| libtiff | 7.09s | 666ms | 10.64x | 6.01s | 52ms | 115.62x |
| fontconfig | 9.95s | 3.76s | 2.65x | 8.79s | 2.49s | 3.53x |
| python@3.13 | 14.35s | 6.49s | 2.21x | 12.67s | 196ms | 64.66x |
| git | 6.44s | 1.47s | 4.38x | 4.78s | 135ms | 35.38x |
| little-cms2 | 7.79s | 745ms | 10.46x | 6.79s | 54ms | 125.67x |
| dav1d | 2.50s | 470ms | 5.32x | 1.45s | 27ms | 53.67x |
| openexr | 9.90s | 762ms | 12.99x | 8.80s | 61ms | 144.21x |
| c-ares | 2.44s | 472ms | 5.18x | 1.48s | 32ms | 46.25x |
| tesseract | 35.42s | 5.20s | 6.82x | 33.95s | 2.88s | 11.79x |
| p11-kit | 8.64s | 5.20s | 1.66x | 7.70s | 34ms | 226.44x |
| imagemagick | 14.91s | 1.29s | 11.51x | 13.81s | 82ms | 168.46x |
| zlib | 2.31s | 423ms | 5.46x | 1.42s | 26ms | 54.65x |
| libx11 | 5.72s | 1.01s | 5.65x | 4.71s | 200ms | 23.57x |
| freetype | 3.66s | 512ms | 7.15x | 2.71s | 30ms | 90.27x |
| protobuf | 3.97s | 691ms | 5.74x | 2.94s | 58ms | 50.69x |
| gnupg | 22.00s | 6.30s | 3.49x | 20.51s | 199ms | 103.09x |
| openjph | 7.80s | 684ms | 11.41x | 6.80s | 54ms | 126.02x |
| libtasn1 | 2.38s | 379ms | 6.27x | 1.47s | 27ms | 54.26x |
| ruby | 16.37s | 8.51s | 1.92x | 15.32s | 989ms | 15.49x |
| gnutls | 15.08s | 5.86s | 2.57x | 13.79s | 136ms | 101.36x |
| expat | 2.34s | 463ms | 5.06x | 1.43s | 26ms | 54.85x |
| libsodium | 2.47s | 533ms | 4.64x | 1.46s | 28ms | 52.18x |
| simdjson | 2.53s | 561ms | 4.52x | 1.45s | 27ms | 53.63x |
| gemini-cli | 30.67s | 7.27s | 4.22x | 28.36s | 748ms | 37.92x |
| libarchive | 5.07s | 594ms | 8.54x | 3.98s | 36ms | 110.50x |
| pyenv | 13.46s | 6.24s | 2.16x | 11.96s | 414ms | 28.88x |
| pixman | 2.35s | 483ms | 4.87x | 1.46s | 28ms | 52.29x |
| curl | 17.31s | 6.95s | 2.49x | 16.21s | 447ms | 36.27x |
| opus | 2.53s | 499ms | 5.08x | 1.49s | 27ms | 55.07x |
| unbound | 13.20s | 6.39s | 2.06x | 11.83s | 390ms | 30.33x |
| cairo | 20.75s | 4.33s | 4.79x | 19.90s | 2.77s | 7.19x |
| pango | 26.81s | 4.30s | 6.23x | 24.00s | 2.88s | 8.34x |
| leptonica | 9.43s | 763ms | 12.36x | 8.52s | 64ms | 133.19x |
| libxcb | 4.90s | 913ms | 5.36x | 4.11s | 141ms | 29.15x |
| jpeg-xl | 13.00s | 814ms | 15.97x | 11.92s | 70ms | 170.33x |
| coreutils | 4.28s | 1.08s | 3.95x | 2.23s | 37ms | 60.16x |
| certifi | 7.81s | 5.40s | 1.45x | 6.97s | 34ms | 204.97x |
| krb5 | 11.11s | 6.25s | 1.78x | 10.02s | 374ms | 26.78x |
| docker | 2.31s | 620ms | 3.73x | 1.21s | 28ms | 43.11x |
| libheif | 10.70s | 901ms | 11.87x | 9.84s | 64ms | 153.73x |
| webp | 4.39s | 936ms | 4.69x | 3.44s | 37ms | 93.00x |
| libxext | 6.72s | 941ms | 7.14x | 6.00s | 207ms | 28.98x |
| libxau | 2.83s | 544ms | 5.20x | 2.02s | 37ms | 54.65x |
| gcc | 11.50s | 3.00s | 3.84x | 9.18s | 69ms | 132.97x |
| bzip2 | 2.29s | 447ms | 5.12x | 1.17s | 27ms | 43.48x |
| libxdmcp | 3.04s | 517ms | 5.87x | 1.87s | 44ms | 42.43x |
| abseil | 3.15s | 706ms | 4.45x | 1.91s | 56ms | 34.12x |
| xcbeautify | 2.31s | 514ms | 4.48x | 1.23s | 46ms | 26.78x |
| libuv | 2.59s | 419ms | 6.17x | 1.54s | 37ms | 41.62x |
| giflib | 2.46s | 432ms | 5.70x | 1.53s | 26ms | 58.73x |
| utf8proc | 2.60s | 341ms | 7.64x | 1.58s | 27ms | 58.56x |
| libxrender | 6.77s | 907ms | 7.46x | 5.58s | 174ms | 32.04x |
| m4 | 2.08s | 455ms | 4.58x | 1.15s | 26ms | 44.15x |
| graphite2 | 2.33s | 443ms | 5.25x | 1.46s | 37ms | 39.46x |
| openjdk | 31.64s | 6.80s | 4.65x | 29.65s | 2.93s | 10.11x |
| uvwasi | 2.99s | 477ms | 6.28x | 2.18s | 41ms | 53.20x |
| libffi | 2.56s | 470ms | 5.45x | 1.55s | 34ms | 45.56x |
| libdeflate | 2.68s | 326ms | 8.23x | 1.55s | 33ms | 47.09x |
| llvm | 26.15s | 6.50s | 4.02x | 9.78s | 191ms | 51.19x |
| aom | 3.18s | 621ms | 5.13x | 2.16s | 33ms | 65.42x |
| lzo | 2.44s | 409ms | 5.98x | 1.46s | 27ms | 53.93x |
| libevent | 11.16s | 6.45s | 1.73x | 9.94s | 348ms | 28.55x |
| libgpg-error | 5.03s | 986ms | 5.10x | 3.93s | 71ms | 55.35x |
| libidn2 | 4.86s | 979ms | 4.96x | 3.86s | 71ms | 54.42x |
| berkeley-db@5 | 4.10s | 1.46s | 2.80x | 2.68s | 56ms | 47.86x |
| deno | 10.73s | 1.24s | 8.64x | 9.21s | 58ms | 158.84x |
| libedit | 2.34s | 457ms | 5.11x | 1.44s | 27ms | 53.41x |
| oniguruma | 2.39s | 489ms | 4.88x | 1.48s | 27ms | 54.93x |

</details>

To reproduce, run `just bench --full results/` on a machine with nothing installed in Homebrew. It resets zerobrew, uninstalls everything it installed in Homebrew, and writes a table to `results/benchmark.md`.

A smaller version runs in CI: the [parity workflow](https://github.com/zerobrewhq/zerobrew/actions/workflows/parity.yml) installs a fixed set of formulae with both tools on the same runner and compares the resulting prefixes byte for byte, and the [timing workflow](https://github.com/zerobrewhq/zerobrew/actions/workflows/timing.yml) times those formulae plus node and python@3.14, cold and warm, with both tools and with the last zerobrew release. 

The nightly timing run fails if zerobrew is less than 1.5x faster than Homebrew cold or 3x warm on any of them, or more than 20% slower than the last release overall.

## A note on how we are achieving these speeds

WRT Homebrew, I want it to be clear that both `brew` and `zerobrew` install the same bottles from *Homebrew's build farm*. 

The difference is what happens after the download: Homebrew runs Ruby to evaluate the formula, unpacks, rewrites paths with `install_name_tool`, and re-signs each binary one at a time, while zerobrew **does the relocation in-process** and links from a content-addressed store, so it pays for the download once and almost nothing after.

**I want it to be clear that we are basically nothing without Homebrew's bottle build farm**. Every package zerobrew installs is one they built, tested and published. We don't compile anything, we don't maintain formulae, and none of these numbers exist without that work. The speed comes from how the artifacts are installed, not from what's in them.

This is a good read, if you're interested in what zerobrew is and isn't: [Standing on the shoulders of Homebrew](https://nesbitt.io/2026/04/14/standing-on-the-shoulders-of-homebrew.html) by Andrew Nesbitt. It makes the point better than I can.

**nanobrew:**

[nanobrew’s](https://github.com/justrach/nanobrew) published numbers time a “warm install” of a package that is already installed, which is an early exit, not an install, and compare against zerobrew 0.1.0. 

Every number above is a real install from an uninstalled state, with the script and the full log in this repo, so you can run it yourself.

## Relationship with Homebrew

zerobrew is more of a performance-optimized client for the Homebrew ecosystem. We rely on:
- Homebrew's formula definitions (homebrew-core)
- Homebrew's pre-built bottles when available
- Homebrew's package metadata and infrastructure

Our innovations focus on:
- Content-addressable storage for deduplication
- APFS clonefiles for zero-overhead copying
- Source build fallback using Homebrew's Ruby DSL

zerobrew is experimental. We recommend running it alongside Homebrew rather than as a replacement, and do _not_ 
recommend purging homebrew and replacing it with zerobrew unless you are absolutely sure about the implications of 
doing so. 

## Project status

<div align="center">
  <a href="https://star-history.com/#zerobrewhq/zerobrew&Date">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=zerobrewhq/zerobrew&type=Date&theme=dark" />
      <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=zerobrewhq/zerobrew&type=Date" />
    </picture>
  </a>
</div>

- **Status:** Experimental, but quite useful. I ([@cachebag](https://github.com/cachebag)) daily drive it myself.
- **Feedback:** If you hit incompatibilities, please open an issue or PR.
- **License:** Dual-licensed under [Apache 2.0](./LICENSE-APACHE.md) OR [MIT](./LICENSE-MIT.md), at your choice. The source build shim and the post-install runner are derived from Homebrew and are covered by its [BSD 2-Clause license](./LICENSE-HOMEBREW).
