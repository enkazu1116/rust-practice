# 環境構築
## 前提
言語自体の環境構築はすでに完了済み

## 作業内容
今回topcoatを使用して開発をしたいと考えているため、そのセットアップをする。

### 実施内容
下記コマンドを実行するとできていそう。

```shell
cargo add topcoat@0.5.0
cargo add tokio@1 --features rt-multi-thread,macros
```
[参考記事](https://github.com/enkazu1116/rust-practice.git)