# test-vscode-ext-serve-run-wasm-client

[hankei6km/test-vscode-ext-serve-run-wasm](https://github.com/hankei6km/test-vscode-ext-serve-run-wasm) のクライアント検証用。

~~IPC(Unix Domain Socket)はとりあえず保留。~~

検証したい項目はだいたい確認できた。

懸念事項
- 遅い 
- 引数は query paraqms で渡すので長い引数は渡せないかもしれない
    - 送信する方も ndjson にして引数もそこに含める。は、遅くなるからあまりやりたくない