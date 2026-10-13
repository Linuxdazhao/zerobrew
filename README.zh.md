<div align="center">

<h2>zerobrew</h2>

<p align="center">
  <a href="README.md">English</a> ·
  <strong>中文</strong>
</p>

[![Lint](https://github.com/zerobrewhq/zerobrew/actions/workflows/ci.yml/badge.svg)](https://github.com/zerobrewhq/zerobrew/actions/workflows/ci.yml)
[![Test](https://github.com/zerobrewhq/zerobrew/actions/workflows/test.yml/badge.svg)](https://github.com/zerobrewhq/zerobrew/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/zerobrewhq/zerobrew?display_name=tag)](https://github.com/zerobrewhq/zerobrew/releases)
[![Discord](https://img.shields.io/badge/Discord-Join-5865F2?logo=discord&logoColor=white)](https://discord.gg/ZaPYwm9zaw)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE-MIT.md)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE-APACHE.md)

<img alt="zerobrew demo" src="./assets/zb-demo.gif" />

<p><strong>zerobrew 为 macOS 和 Linux 上的 Homebrew 软件包带来了类似 uv 的架构。</strong></p>

</div>

## 安装 (Install)

```bash
curl -fsSL https://zerobrew.rs/install | bash
```

安装程序会更新你的 shell 配置。完成后，重启终端，或运行它打印的 `source` 命令。

或通过 Homebrew 安装：

```bash
brew install zerobrewhq/zerobrew/zerobrew
```

## 更新 zerobrew (Update zerobrew)

如果使用独立安装脚本，重新运行：

```bash
curl -fsSL https://zerobrew.rs/install | bash
zb --version
```

如果通过 Homebrew 安装：

```bash
brew update && brew upgrade zerobrew
```

如果你是从旧的 `lucasgelfond/zerobrew` tap 安装的，它已不再更新。请切换到新的 tap：

```bash
brew uninstall zerobrew
brew untap lucasgelfond/zerobrew
brew install zerobrewhq/zerobrew/zerobrew
```

`zb update` 只刷新软件包元数据。`zb upgrade` 升级通过 zerobrew 安装的软件包。它们都不会更新 `zb` 二进制文件本身。

## 快速开始 (Quick start)

需要使用 `brew` 命令时，可以手动启用兼容入口：

```bash
zb install-alias brew
brew install jq
zb install-alias brew --remove
```

启用后会在 `zb` 所在目录创建 `brew -> zb-brew`，该目录需要在 PATH 中。
也可以在安装时启用：

```bash
curl -fsSL https://zerobrew.rs/install | bash -s -- --brew-compat
```

如果 PATH 或 Homebrew 的标准安装位置已有 Homebrew 或其他 `brew` 入口，
启用会被拒绝，不会覆盖已有文件。移除命令只删除 zerobrew 自己创建的软链接。
缺少 `zb-brew` 时，需要重新安装 zerobrew。

兼容入口支持 `install`、`uninstall`、`list`、`info`、`outdated`、`upgrade`、
`doctor`、`bundle` 和 `update`，参数与输出沿用 `zb`。
`cleanup` 对应 `zb gc`，只清理未引用的 store 条目；`update` 只刷新软件包索引，
不会更新 zerobrew 本身。`search`、`services`、`tap` 等命令，以及 `--cask`、
`list --formula` 等参数暂不支持，会明确报错。
`zb migrate` 需要 PATH 中的真实 Homebrew，不能通过该兼容入口执行迁移。

```bash
zb install jq                   # 安装单个软件包
zb install wget git             # 安装多个软件包
zb bundle                       # 从 Brewfile 安装
zb bundle install -f myfile     # 从自定义文件安装
zb bundle dump                  # 将已安装的软件包导出到 Brewfile
zb bundle dump -f out --force   # 导出到自定义文件（覆盖）
zb uninstall jq                 # 卸载单个软件包
zb outdated                     # 列出有新版本可用的软件包
zb upgrade                      # 升级所有已过期的软件包
zb upgrade jq wget              # 升级指定的软件包
zb reset                        # 卸载所有内容
zb gc                           # 垃圾回收未使用的存储条目
zbx jq --version                # 在不链接的情况下运行
```

自动初始化（`--auto-init` 或 `ZEROBREW_AUTO_INIT=true`）和 `zb reset`
会创建数据目录，不会修改 shell 配置。需要配置 shell 时，显式运行 `zb init`；
传入 `zb init --no-modify-path` 则保留现有配置。

## 性能快照 (Performance snapshot)

<div align="center">

| 软件包 | Homebrew (冷启动) | ZB (冷启动) | 冷启动加速 | Homebrew (热启动) | ZB (热启动) | 热启动加速 |
|---------|-----------------|-----------|--------------|-----------------|-----------|--------------|
| **总体 (100 个软件包)** | 782s | 216s | **3.6x** | 650s | 27.7s | **23x** |
| python@3.14 | 17.10s | 6.44s | 2.7x | 15.33s | 440ms | 34.8x |
| node | 28.37s | 7.24s | 3.9x | 26.80s | 743ms | 36.1x |
| tesseract | 35.42s | 5.20s | 6.8x | 33.95s | 2.88s | 11.8x |
| openjdk | 31.64s | 6.80s | 4.7x | 29.65s | 2.93s | 10.1x |
| llvm | 26.15s | 6.50s | 4.0x | 9.78s | 191ms | 51.2x |

</div>

测试于 2026-10-12，使用 zerobrew 0.4.0+dev 和 Homebrew 7.0.9，环境为 macOS 26.6.2、MacBook Pro（M3 Pro，18 GB 内存），下载带宽约 374 Mbit/s。测试开始时 Homebrew 中未安装任何软件包。

- **冷启动**：该软件包及其所有依赖均已卸载，下载缓存为空。
- **热启动**：该软件包及其所有依赖均已卸载，保留冷启动时的下载缓存。
- 按中位数计算，单个软件包冷启动快 4.8 倍，热启动快 52 倍。按软件包看，冷启动加速从 1.4 倍（ca-certificates）到 16 倍（jpeg-xl），热启动从 3.5 倍（fontconfig）到 232 倍（ca-certificates）。100 个软件包中有 17 个热启动快 100 倍以上。
- 两个工具在这里都会运行 post-install 步骤。ca-certificates 的步骤会根据系统钥匙串重建 CA 证书包，两边都要花大约 5 秒，而几乎所有软件包都依赖它，所以它决定了表中大部分软件包的冷启动下限。zerobrew 会缓存这个证书包，这就是 ca-certificates 热启动只需 27ms 的原因。fontconfig 的 `fc-cache` 也是同样的情况，但尚未缓存（[#448](https://github.com/zerobrewhq/zerobrew/issues/448)），这也是 fontconfig、harfbuzz、cairo 和 pango 热启动不到 10 倍的原因。
- 冷启动受网络带宽限制。同一测试在约 69 Mbit/s 的网络上（[results/2026-10-07](results/2026-10-07/benchmark.md)）冷启动为 3.3 倍，当时尚未运行 post-install。zerobrew 运行 post-install 步骤之前的测试见 [results/2026-10-08](results/2026-10-08/benchmark.md)：冷启动 6.6 倍、热启动 68 倍，当时 Homebrew 做了这部分工作而 zerobrew 没有。
- 热启动会占用更多磁盘。卸载后 brew 只保留 bottle，我们则同时保留 bottle 和存储区中已解压的 keg。这正是重新安装只需克隆而不必重建的原因。`zb gc` 会清理它。我会对同一组软件包测量两者的占用并把数字放在这里。

<details>
<summary>完整结果</summary>

| 软件包 | Homebrew (冷启动) | ZB (冷启动) | 冷启动加速 | Homebrew (热启动) | ZB (热启动) | 热启动加速 |
|---------|-----------------|-----------|--------------|-----------------|-----------|--------------|
| **总体 (100 个软件包)** | 781.58s | 215.64s | **3.62x** | 650.09s | 27.74s | **23.43x** |
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

如需复现，请在 Homebrew 中未安装任何软件包的机器上运行 `just bench --full results/`。该命令会重置 zerobrew，卸载其在 Homebrew 中安装的所有软件包，并将表格写入 `results/benchmark.md`。

CI 中会运行一个较小的版本：[parity workflow](https://github.com/zerobrewhq/zerobrew/actions/workflows/parity.yml) 在同一台 runner 上用两个工具安装一组固定的 formula，并逐字节比较生成的 prefix；[timing workflow](https://github.com/zerobrewhq/zerobrew/actions/workflows/timing.yml) 则对这些 formula 以及 node 和 python@3.14 分别用两个工具和上一个 zerobrew 发布版计时冷启动和热启动。

每晚的计时运行中，如果 zerobrew 在任一 formula 上冷启动不到 Homebrew 的 1.5 倍快、热启动不到 3 倍快，或整体比上一个发布版慢 20% 以上，就会失败。

## 关于这些速度是怎么来的 (A note on how we are achieving these speeds)

关于 Homebrew，我想说清楚：`brew` 和 `zerobrew` 安装的是来自 *Homebrew 构建农场* 的同一批 bottle。

区别在于下载之后：Homebrew 运行 Ruby 来求值 formula、解压、用 `install_name_tool` 重写路径，并逐个重新签名每个二进制文件；而 zerobrew **在进程内完成重定位**，并从内容寻址存储中链接，所以下载只付一次代价，之后几乎不再有开销。

**我想说清楚，没有 Homebrew 的 bottle 构建农场，我们基本什么都不是**。zerobrew 安装的每一个软件包都是他们构建、测试并发布的。我们不编译任何东西，不维护 formula，没有他们的工作就没有上面这些数字。速度来自工件的安装方式，而不是工件本身。

如果你想了解 zerobrew 是什么、不是什么，这篇文章值得一读：Andrew Nesbitt 的 [Standing on the shoulders of Homebrew](https://nesbitt.io/2026/04/14/standing-on-the-shoulders-of-homebrew.html)。他说得比我好。

**nanobrew：**

[nanobrew](https://github.com/justrach/nanobrew) 公布的数字计的是已安装软件包的"热安装"，那是提前退出而不是安装，而且对比的是 zerobrew 0.1.0。

上面的每一个数字都是从未安装状态开始的真实安装，脚本和完整日志都在本仓库中，你可以自己跑。

## 与 Homebrew 的关系 (Relationship with Homebrew)

zerobrew 更像是一个针对 Homebrew 生态系统进行性能优化的客户端。我们依赖于：
- Homebrew 的 formula 定义 (homebrew-core)
- Homebrew 提供的预构建 bottle（如果可用）
- Homebrew 的软件包元数据和基础设施

我们的创新重点在于：
- 用于去重的基于内容的寻址存储 (Content-addressable storage)
- 用于零开销复制的 APFS clonefiles
- 使用 Homebrew 的 Ruby DSL 的源码编译回退 (Source build fallback)

zerobrew 处于实验阶段。我们建议将其与 Homebrew 并行运行，而不是作为替代品。除非您完全确定其影响，否则**不**建议清除 Homebrew 并将其替换为 zerobrew。

## 项目状态 (Project status)

<div align="center">
  <a href="https://star-history.com/#zerobrewhq/zerobrew&Date">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=zerobrewhq/zerobrew&type=Date&theme=dark" />
      <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=zerobrewhq/zerobrew&type=Date" />
    </picture>
  </a>
</div>

- **状态：** 处于实验阶段，但相当好用。我（[@cachebag](https://github.com/cachebag)）每天都在用它。
- **反馈：** 如果遇到不兼容问题，请提出 issue 或 PR。
- **许可证：** 根据您的选择，在 [Apache 2.0](./LICENSE-APACHE.md) 或 [MIT](./LICENSE-MIT.md) 下获得双重许可。源码编译 shim 和 post-install 运行器派生自 Homebrew，适用其 [BSD 2-Clause 许可证](./LICENSE-HOMEBREW)。
