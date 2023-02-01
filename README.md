# PC Status Client - Rust
PCの状態を取得し，[PC Status](https://pc-stats.eov2.com/)に送信，表示するツールです。

![image](https://cdn.discordapp.com/attachments/916525261409898527/1070176168948539452/0UAAAAASUVORK5CYII.png)

## 注意
**ツールの性質上，以下の内容が他者に誰でも見られる状態で送信されるため，少しでも不快感を感じる人であれば使用しないでください。**\
個人情報に繋がるような情報は**ホスト名を除き**送信される事はありませんが，必要に応じてPCのホスト名を変更するか，或いは `.env` 内に以下のKeyを追加してください。

```env
HOSTNAME=ホスト名として表示させたい文字列
```

## 送信，表示内容
1. PCのホスト名 (e.g. `assault-e5dmts`)
    - ホスト名に個人情報などが含まれている場合は使用しない事を**強く**推奨します。
    - 64文字まで受け付けますが，32文字目以降は ... で隠されます。\
      マウスオーバーで全て表示されます。
2. OSのバージョン (e.g. `Windows 10 Home(Windows_NT win32 x64 10.0.19044)`)
3. CPU名，CPU使用率 (全体, コアごと) (e.g. `AMD Ryzen 5 3500 6-Core Processor`)
4. メモリ使用量
5. ストレージ占有量 (実行しているrootを参照)
6. GPU使用率 (NVIDIAのGPUのみ)
7. 起動時間
8. Nodeのバージョン
9. Load Average (Linuxのみ)

## 使い方

Coming soon...