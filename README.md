nazrin
======
小さな小さなユーティリティ

## 概要

本ユーティリティはバッチで自動化スクリプトを書く際に、バッチだけでは不足する機能を単独で補完する事を目的としています。

外部ランタイムに依存しない単一の実行ファイルとして動作するため、実行ファイルを 1 つ置くだけで利用できます。

## コマンド体系

サブコマンドは「機能グループ + 操作」の 2 階層構成になっています。

```
nazrin <グループ> <操作> [オプション]
```

|コマンド|説明|
|:-|:-|
|`zip compress`|指定されたディレクトリを ZIP アーカイブにします。|
|`zip expand`|指定された ZIP アーカイブを展開します。|
|`base64 encode`|指定されたファイルを base64 エンコードします。|
|`base64 decode`|指定された base64 ファイルをデコードします。|
|`unleash`|指定されたファイルを次回再起動時に削除するよう予約します。(Windows 専用)|

ヘルプは各階層で参照できます。

```
nazrin --help
nazrin zip --help
nazrin zip compress --help
```

## 使い方

### zip compress

指定されたディレクトリを ZIP アーカイブに圧縮します。圧縮方式は deflate です。

|オプション|必須|説明|
|:-|:-|:-|
|`-i`, `--input <PATH>`|必須|圧縮対象のディレクトリ。|
|`-o`, `--output <PATH>`|必須|出力する ZIP アーカイブのパス。|

```
nazrin zip compress -i C:\work\logs -o C:\backup\logs.zip
```

アーカイブ内のパスは入力ディレクトリからの相対パスになります。上記の例では、
`C:\work\logs\app\today.log` はアーカイブ内に `app/today.log` として格納されます
(入力ディレクトリ名の `logs` は含まれません)。サブディレクトリは再帰的にたどられます。

### zip expand

指定された ZIP アーカイブを展開します。

|オプション|必須|説明|
|:-|:-|:-|
|`-i`, `--input <PATH>`|必須|展開する ZIP アーカイブのパス。|
|`-o`, `--output <PATH>`|必須|展開先のディレクトリ。|

```
nazrin zip expand -i C:\backup\logs.zip -o C:\restore\logs
```

展開先の中間ディレクトリは必要に応じて自動的に作成されます。
アーカイブ内に `..` を含むエントリなど、展開先の外に出てしまうパスが含まれていた場合、
そのエントリは展開されずにスキップされます (ディレクトリトラバーサル対策)。

### base64 encode

ファイルを base64 エンコードしてテキストファイルとして出力します。

|オプション|必須|説明|
|:-|:-|:-|
|`-i`, `--input <PATH>`|必須|エンコード対象のファイル。|
|`-o`, `--output <PATH>`|必須|出力先のファイル。|

```
nazrin zip compress -i C:\work\logs -o %TEMP%\logs.zip
nazrin base64 encode -i %TEMP%\logs.zip -o C:\out\logs.zip.txt
```

出力される形式は標準の base64 (RFC 4648、`+` `/` を使用、`=` パディングあり) です。
**改行は一切挿入されず、全体が 1 行になります。末尾の改行もありません。**
Windows 標準の `certutil -encode` とは異なり `-----BEGIN CERTIFICATE-----` のような
ヘッダー行は付きません。

### base64 decode

base64 ファイルをデコードして元のファイルに戻します。

|オプション|必須|説明|
|:-|:-|:-|
|`-i`, `--input <PATH>`|必須|デコード対象の base64 ファイル。|
|`-o`, `--output <PATH>`|必須|出力先のファイル。|

```
nazrin base64 decode -i C:\out\logs.zip.txt -o %TEMP%\logs.zip
nazrin zip expand -i %TEMP%\logs.zip -o C:\restore\logs
```

入力は改行を含まない 1 行の base64 である必要があります。
改行やヘッダー行を含むファイル (`certutil -encode` の出力など) を渡すと
`Invalid symbol 13, offset 76.` のようなエラーになり、終了コード 1 で失敗します。
`nazrin base64 encode` の出力はそのままデコードできます。

### unleash (Windows 専用)

指定されたファイルを **次回 OS 再起動時に削除する** よう OS に予約します。
実行中のプロセスがロックしていて通常は削除できないファイルを、
再起動のタイミングで確実に消したい場合に使用します。

|オプション|必須|説明|
|:-|:-|:-|
|`-t`, `--target <PATH>`|必須|削除を予約する対象のパス。|
|`-r`, `--recursive`|任意|対象がディレクトリの場合、配下を再帰的に予約します。|

```
nazrin unleash -t C:\work\locked.dll
nazrin unleash -t C:\work\tempdir -r
```

内部では Win32 API の `MoveFileExW` を `MOVEFILE_DELAY_UNTIL_REBOOT` フラグ付きで
呼び出しています。`-r` を指定した場合は、ディレクトリ自身より先に配下のエントリが
予約されるよう、末端から順に処理されます。

> **注意**
>
> - このコマンドは実行した時点では何も削除されず、**次回再起動時に削除が実行されます。**
>   予約は取り消せないため、対象パスをよく確認してから実行してください。
> - 予約の登録には管理者権限が必要な場合があります。

## 終了コード

|終了コード|意味|
|:-|:-|
|`0`|正常終了。|
|`1`|処理中にエラーが発生。エラーメッセージは標準エラー出力に出力されます。|
|`2`|コマンドラインの解析に失敗 (未知のサブコマンド、必須オプションの不足など)。|

バッチからはこのように判定できます。

```bat
nazrin zip compress -i C:\work\logs -o C:\backup\logs.zip
if errorlevel 1 (
    echo 圧縮に失敗しました
    exit /b 1
)
```

なお、出力先の親ディレクトリが存在しない場合はエラーになります
(`zip expand` の展開先のみ自動作成されます)。あらかじめ `mkdir` しておいてください。

## ビルド

Rust (edition 2024 対応版) のツールチェインが必要です。

```
cargo build --release
```

`target/release/nazrin` (Windows では `nazrin.exe`) が生成されます。
Windows の MSVC ターゲットでは CRT を静的リンクする設定になっているため、
Visual C++ 再頒布可能パッケージがない環境でもそのまま実行できます。

## ライセンス

[LICENSE](LICENSE) を参照してください。
