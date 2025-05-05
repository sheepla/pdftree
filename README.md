<div align="center">

# 🌲 pdftree

</div>

<div align="center">

**pdftree** is a command line tool to extract outline tree from PDF file written in Rust.

</div>

## Usage

Run the program with the path to the PDF file as the first argument. Note that, of course, if the file has no outline, it will fail.

```
Usage: pdftree [OPTIONS] <PATH>

Arguments:
  <PATH>

Options:
  -j, --json  Output as JSON
  -h, --help  Print help
```

For example, `pdftree ./Professional_IPv6.pdf` will result in output like this.

<details>
<summary> Command line output </summary>

```
- 表紙
- 扉裏
- 序文
- 目次
  - 更新履歴
- 第I部 インターネットとIPv6の概要
  - 第1章 インターネット概要
    - 1.1 IPはパケット交換技術
    - 1.2 層に分かれるネットワーク
    - 1.3 トランスポート層の役割
    - 1.4 オープンなプロトコルとRFC
  - 第2章 IPv6概論
    - 2.1 IPv6アドレス
    - 2.2 IPv6アドレスのテキスト表記
    - 2.3 IPv6アドレス体系
    - 2.4 IPv6におけるユニキャストアドレスの構成要素
    - 2.5 IPv6基本プロトコル
    - 2.6 ICMPv6の役割
    - 2.7 近隣探索プロトコル
    - 2.8 IPv6アドレスの自動設定
    - 2.9 1つのネットワークインターフェースに複数のIPv6アドレスが設定される
    - 2.10 IPv6とIPv4の違い
    - 2.11 IPv6対応とは
    - 2.12 IPv6インターネットとIPv4インターネットを同時に使う
    - 2.13 IPv4とIPv6の共存技術
- 第II部 IPv6プロトコルとその周辺技術
  - 第3章 IPv6アドレス体系
    - 3.1 IPv6アドレスの種類
    - 3.2 IPv6アドレス空間の使い方はIANAが管理している

...
```

</details>

Run with the `--json` option to output the outline in JSON. For example, `pdftree --json . /Professional_IPv6.pdf` will produce the following output.

<details>
<summary>Command line output</summary>

```json
[
  {
    "title": "表紙",
    "children": []
  },
  {
    "title": "扉裏",
    "children": []
  },
  {
    "title": "序文",
    "children": []
  },
  {
    "title": "目次",
    "children": [
      {
        "title": "更新履歴",
        "children": []
      }
    ]
  },
  {
    "title": "第I部 インターネットとIPv6の概要",
    "children": [
      {
        "title": "第1章 インターネット概要",
        "children": [
          {
            "title": "1.1 IPはパケット交換技術",
            "children": []
          },
          {
            "title": "1.2 層に分かれるネットワーク",
            "children": []
          },
          {
            "title": "1.3 トランスポート層の役割",
            "children": []
          },
          {
            "title": "1.4 オープンなプロトコルとRFC",
            "children": []
          }
        ]
      },
      {
        "title": "第2章 IPv6概論",
        "children": [
          {

...
```

</details>

## Installation

Clone this repository then run `cargo install --path=.` .

## Thanks

- [lopdf](https://github.com/J-F-Liu/lopdf): A Rust library for PDF document manipulation. 

