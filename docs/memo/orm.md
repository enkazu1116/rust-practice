# ORMを使ってみる
## 選定候補
- Diesel
- SeaOrm
- SQLx
- Toasty

### Diesel
スキーマ定義・モデル定義をそれぞれ行う。
ウィンドウ関数にも対応していて、不便はしないと考えられる。
asyncには非対応。diesel-asyncで非同期I/Oをサポートすることができる。

[解説参考](https://zenn.dev/bflg/scraps/8a3b4bff853326)
[公式](https://diesel.rs/)

### SeaOrm
RailsのActiveRecordに近いみたい。
使用したことがないので、考えから理解する必要がありそう。
ドキュメントは見やすい印象を受ける。

[解説参考](https://zenn.dev/collabostyle/articles/0641d73f776d80)
[公式](https://www.sea-ql.org/SeaORM/)

### SQLx
ORMではないそう。
他と異なり、SQLを自前で書けるので複雑なクエリを書きやすい。

[解説記事_使用例](https://zenn.dev/pharmax/articles/7f1cb24dbd5743)
[解説記事_深掘り](https://zenn.dev/collabostyle/articles/9747c20a516f34)

### Toasty
モデル定義 -> スキーマ生成でデータベースに反映される。
開発体験としては、良さそう。
NoSQLにも対応しているので、試す価値はあり。
ただし、複雑なクエリ定義は難しいらしい。

[選定参考](https://zenn.dev/pharmax/articles/7f1cb24dbd5743)
[解説参考](https://zenn.dev/uniquevision/articles/345368d3188b98)
[公式ガイド](https://tokio-rs.github.io/toasty/0.10.0/guide/querying-records.html)

## 選定
ToastyはTopcoatの作成チームが作っているORMだが、まだ成熟していない。
SeaORMがドキュメントが一番見やすく充実しているように見え、実例も多い。
また、Active Recordについて理解を促進することができるので、今回のRustの練習に合わせて
一石二鳥なので試してみるということで採用する。