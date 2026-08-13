# Pinkeybd RMK firmware

Seeed XIAO nRF52840を2枚使うPinkeybd向けの、RMK 0.8系ファームウェアです。右側をセントラルとしてUSB HID/BLE HIDとPMW3610トラックボールを提供し、左側はBLEペリフェラルとしてキーとエンコーダを転送します。BLEプロファイルは6個です。

## 配線

- Charlieplex: `D1, D2, D3, D6, D7, D8`（両半分共通）
- 左エンコーダ: `D4=A`, `D5=B`（Peripheral）
- 右PMW3610: `D5=SCLK`, `D4=SDIO`, `D9=NCS`, `D10=MOT`（Central）

GPIOはXIAO BLEの実ピンに変換済みです。6本を入力/出力に切り替えるため、通常マトリクス用のGPIO設定ではなくRMKの`BidirectionalMatrix`を使っています。

## ビルド

Rustの組み込みターゲットとフラッシュ用ツールを用意した後、両方を同じRMKバージョンでビルドして書き込みます。

```powershell
rustup target add thumbv7em-none-eabihf
cargo build --release --bin pinkeybd-left
cargo build --release --bin pinkeybd-right
```

XIAOのUF2ブートローダーを使用する場合は、生成されたELFを`cargo-binutils`と`cargo-hex-to-uf2`でUF2へ変換してください。初めて左右を組にする場合は、左右とも既存のBLEペアリング情報を消去してから同じリビジョンを書き込んでください。

`memory.x`はAdafruit UF2ブートローダー用です。SWDでブートローダーなしに書き込む場合は、FLASH開始を`0x00000000`、RAM開始を`0x20000000`に変更してください。
