# PC Status Client - Rust

PC の状態を取得し、[PC Status](https://pc-stats.eov2.com/)に送信、表示するツールです。

![Preview - Overview](https://github.com/eoeo-org/pcsc-rs/assets/34514603/79496a8d-cea4-457e-b307-52cce7a55f33)
| ![Preview - Focus to Windows](https://github.com/eoeo-org/pcsc-rs/assets/34514603/2cc271bc-c24e-4446-bffe-b6d7cb5c7fbd) | ![Preview - Focus to Linux](https://github.com/eoeo-org/pcsc-rs/assets/34514603/c7a28a39-f7d8-4d9f-bec9-3ac845a2037e) |
|---|---|

## 注意

**ツールの性質上、以下の内容が他者に誰でも見られる状態で送信されるため、少しでも不快感を感じる場合は使用しないでください。**\
個人に繋がるような情報は**ホスト名を除き**送信される事はありませんが、必要に応じて PC のホスト名を変更するか、或いは `.env` 内に以下の Key を追加してください。

```env
HOSTNAME=ホスト名として表示させたい文字列
```

## 送信、表示内容

1. PC のホスト名 (e.g. `assault-e5dmts`)
   - ホスト名に個人情報などが含まれている場合は上記[注意](#注意)を参考にホスト名を変更してください。
   - 64 文字まで受け付けますが、32 文字目以降は ... で隠されます。\
     マウスオーバーで全て表示されます。
2. OS のバージョン (e.g. `Windows 10 Home(Windows_NT win32 x64 10.0.19044)` or `Windows 10 (19045)`)
3. CPU 名、CPU 使用率 (全体, コアごと) (e.g. `AMD Ryzen 5 3500 6-Core Processor`)
4. 物理メモリ使用量、スワップメモリ使用量
5. マウントされているストレージ使用量 (実行されている root を参照)
6. GPU 使用率、GPU メモリ使用量 (NVIDIA GPU のみ)
7. 連続起動時間
8. Load Average (Linux のみ)

## 対応環境

最近の Windows (Server 含む), macOS, Linux であれば動作すると思います。\
もし動作しない場合は [Issues](https://github.com/eoeo-org/pcsc-rs/issues) から報告をお願いします。

## 使い方

1. [リリースページ](https://github.com/eoeo-org/pcsc-rs/releases)から使用する環境に合った最新のリリースをダウンロードしてください。
2. 適当なフォルダに保存し、同じフォルダに `.env` ファイルを作成して以下の Key を追加してください。

```env
PASS=npU7pmkkYfuUdKfqzm2BtDfBPEe4pizrXyPVj8Fby3KaUtehNu3ToDtM8uEdGBr3AS9LRUkZixtZxuKTvsL2e4BVrfzWWG7RqqVThLWsVLHLaJJ8ekeGuHtLBkfZpBtv
```

3. ダウンロードしたリリースを実行してください。\
   macOS, Linux 環境では予め `$ chmod +x <ファイル名>` で実行権限を付与する必要があります。
4. [PC Status](https://pc-stats.eov2.com/)にアクセスし、自分の PC が表示されていれば完了です。

必要に応じて `pcsc-rs.exe` のショートカットを `shell:startup` に追加すれば、PC と同時に起動するようになります。

Linuxの場合、`sudo install -D --no-target-directory pcsc-rs-* /usr/local/bin/pcsc-rs`を実行し、Systemdに登録します。\
`sudo --preserve-env=EDITOR systemctl edit --force --full pcsc-rs.service`
```
[Unit]
Description=PCStatus Client
After=network-online.target

[Service]
Environment="PASS=npU7pmkkYfuUdKfqzm2BtDfBPEe4pizrXyPVj8Fby3KaUtehNu3ToDtM8uEdGBr3AS9LRUkZixtZxuKTvsL2e4BVrfzWWG7RqqVThLWsVLHLaJJ8ekeGuHtLBkfZpBtv"
Environment="PCSC_UPDATED=terminate"
ExecStart=/usr/local/bin/pcsc-rs
Restart=always

[Install]
WantedBy=network-online.target
```
```sh
sudo systemctl enable --now pcsc-rs
```

## その他の設定

- `PCSC_UPDATED`

  更新処理後の動作の設定

  | 値          | 説明                      |
  | ----------- | ------------------------- |
  | `none`      | なにもしない (デフォルト) |
  | `terminate` | 終了する                  |
  | `restart`   | 再起動する                |
