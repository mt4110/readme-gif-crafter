# README Gif Crafter (rgc) 
English version is available at [README_EN.md](README_EN.md).

MP4 録画ファイルから、README 用に最適化された GIF デモを生成する CLI ツールです。
## デモ画面
![Demo](./output.gif)

## 特徴

- **シンプルなデフォルト**: `rgc input.mp4` で標準的な 800px 幅の GIF を生成。
- **高画質・軽量**: `ffmpeg` のパレット生成機能等を使い、高品質かつサイズを抑えた GIF を作成。
- **Markdown 出力**: 貼り付けるだけの Markdown スニペットを標準出力。
- **設定可能**: `.readme-gif.toml` で `github` (最大 10MB) や `mini` (最大 5MB) などのプリセットを管理。

## インストール

## インストール

### Nix を使用する場合 (推奨)

Nix 環境があれば、依存関係 (`ffmpeg` 等) も自動で揃うため最も簡単です。

```bash
nix develop
# 開発シェルに入った状態で:
cargo run --release -- ./assets/demo.mp4
```


またはワンショット実行:
```bash
nix run . -- input.mp4
```

### ソースコードからビルド

`ffmpeg` がインストールされている必要があります。

```bash
git clone https://github.com/yourusername/readme-gif-crafter.git
cd readme-gif-crafter
cargo install --path .
```

## 開発・テスト

### テストの実行
全てのテストを実行するには以下のコマンドを使用します。

```bash
cargo test
```

### デモ生成の自動化
`script/update-demo.sh` を使用すると、デモの再生成と README の更新を一括で行えます。

```bash
./script/update-demo.sh
```

### 環境について
- **Rust Version**: `1.82.0` (Pinned via `.mise.toml` & `flake.nix`)
- **Mise**: `mise` を使用している場合、`.mise.toml` により自動でバージョンが固定されます。
- **Nix**: `flake.nix` も同じバージョン (`1.82.0`) を提供するため、どちらを使っても整合性が保たれます。

## 使い方

```bash
# 基本的な使用法（幅 800px, 15fps）
rgc demo.mp4

# プリセット指定
rgc --preset mini demo.mp4

# カスタム指定
rgc demo.mp4 --width 1024 --fps 30 --output my-demo.gif
rgc demo.mp4 --width 1024 --fps 30 --output my-demo.gif
```

<!-- rgc:start -->



<!-- rgc:end -->

## 設定ファイル

プロジェクトルートに `.readme-gif.toml` を作成してプリセットを定義できます。

**サンプル**: [examples/sample-config.toml](examples/sample-config.toml)

```toml
# --preset を指定しなかった場合に使われるプリセット名
default_preset = "github"

# "github" という名前のプリセット定義
[github]
width = 800         # リサイズ幅 (アスペクト比は維持)
fps = 15            # フレームレート
max_size_mb = 10.0  # (計画中) 目標ファイルサイズ (MB)

# "mini" という名前のプリセット定義
[mini]
width = 480
fps = 12
max_size_mb = 5.0
```

<!-- config-demo:start -->
<!-- Config demo placeholder -->
<!-- config-demo:end -->

### 項目説明

- `default_preset`: フラグなしで `rgc input.mp4` を実行した際に使用されるプリセット名。
- `[section_name]`: `--preset section_name` で指定するプリセット名を定義します。
- `width`: 出力画像の幅（ピクセル）。高さはアスペクト比に合わせて自動計算されます。
- `fps`: 秒間フレーム数。数値を下げるとファイルサイズを大幅に削減できます。
- `max_size_mb`: 目標ファイルサイズ。このサイズに収まるように画質調整を行う予定です（ロジック実装中）。

## ロードマップ & TODO

- [ ] **Windows サポート**: 現在は macOS/Linux での動作を確認しています。
- [ ] **高度なサイズ最適化**: `max_size_mb` のロジックを改善し、ビットレート調整なども行う。
- [ ] **インタラクティブモード**: クロップ範囲を対話的に選択できる機能。
- [ ] **プレビュー機能**: 生成前に設定を確認できる機能。

## ライセンス

MIT License
