[english ver](README-en.md)

> このREADMEはAIによって生成された文章に手直しを入れた物です
> 最終更新日：250731

# `oso` — 実験的aarch64向け純Rust製OS

**`oso`** は、Rustの型安全性と抽象性を最大限に活かしながら低レイヤにおける直接的なハードウェア制御を追求した、外部ツール(QEMU, Rust言語自体等)以外は完全自作のOSです。
独自のUEFIブートローダからマクロ駆動のカーネル設計に至るまで、低レイヤー開発でも抽象を追い求める事を目標としています。
また、まだまだネット上の資料が少ないaarch64を主要ターゲットとしています。
これは

- 情報量としてはマイナーだがポテンシャルの高い領域の一つの先駆け・参考資料となること
- 開発者自身(私の事)の自走能力を鍛える

これらの理由によります。

## QuickStart

コマンド実行の前に[必要なツール](#build)がインストールされている事を確認してください

```bash
git clone https://github.com/sugiura-hiromiti/oso.git
cd oso
cargo xt
```

## 開発哲学・特徴

- [x] aarch64向け
- [x] pure Rust
- [x] no dependencies
  - 開発補助クレートである`xtask`では外部クレートを利用しています
  - その他、自分の技術的好奇心を優先する為に以下の用途で外部クレートを利用しています
    - webスクレイピング：仕様書を元にproc macroで実装を自動生成する
- [x] 標準に従順
  - デファクトスタンダートをリスペクトし、独自規格が0になる様に開発されています
- [x] 積極的な再発明
  - 既存の実装の写経はせず一次情報(仕様書やリファレンス)を元に0からコードを構築しています
  - 開発の中でOSの役割や可能性を再解釈し、OSに出来ることとRustに出来ることを平らな地平から観測する為です
- [x] Rustの高次な機能を積極的に利用
  - 既存のプロジェクトが見落として来たOS開発におけるRust特有の利点を探索する為です

## プロジェクト構成

このリポジトリは、複数のクレートから構成される [Cargoワークスペース](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) です

### `oso_kernel`

カーネル本体を構成するクレート

**特徴**

- nightly Rust機能を多数利用
- モジュール構成：
  - `app`：アプリ実行系
  - `base`：基本ライブラリ
  - `driver`：デバイス制御
- 他クレートとの連携：
  - `oso_no_std_shared`：no_std環境での共有ライブラリ
  - `oso_proc_macro`：マクロによる自動コード生成
  - `oso_error`：エラー処理体系

### `oso_loader`

独自実装のUEFI対応ブートローダ。

- ELF形式のカーネル読み込み機能
- グラフィックス機能サポート（RGB/BGR/Bitmask形式）
- 使用クレート：
  - `oso_no_std_shared`
  - `oso_proc_macro`
  - `oso_error`

### `xtask`

開発者向け補助ツール群。

- QEMU・UEFIターゲットのビルド補助
- 起動用スクリプト
- デプロイやテストの自動化処理など

### 補助クレート一覧

| クレート名             | 説明                                                              |
| ---------------------- | ----------------------------------------------------------------- |
| `oso_no_std_shared`    | no_std環境で共有される基本的なデータ構造とユーティリティを提供    |
| `oso_proc_macro_logic` | 手続きマクロの実装内部ロジックとそのテスト                        |
| `oso_proc_macro`       | カーネルの構造体やパーサ・テスト生成を支援するマクロ群            |
| `oso_error`            | 共通エラー型とエラーハンドリングロジック                          |
| `oso_dev_util`         | 開発ツール間で共有される汎用コード                                |

## Build

**前提条件**：

- nightly Rust
- QEMU
- macOS(`hdiutil`コマンドを利用している為　マルチ対応予定)

**実行例**：

```bash
# 各クレートのビルド・バイナリのマウント・QEMUの実行
cargo xt

# x86も部分的にサポートしています
cargo xt -86
```

## 機能

### グラフィックス機能

- RGB、BGR、Bitmask形式のピクセルフォーマットサポート
- Block Transfer Only (BLT) モード（デフォルト）
- UEFI Graphics Output Protocol (GOP) 対応

### アーキテクチャサポート

- **主要ターゲット**: aarch64 (ARM64)
- **部分サポート**: x86_64

## ライセンス

MIT OR Apache-2.0

## 貢献

このプロジェクトは実験的な性質を持ちますが、コントリビューションを歓迎します。
Issue や Pull Request をお気軽にお送りください。
