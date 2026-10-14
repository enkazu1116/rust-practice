# モデルの作成
## ユーザーモデルの作成
作ってみて、動かしてみないことには理解が深まらないので一旦作成してみる。
[参考記事](https://zenn.dev/collabostyle/articles/031dff7f07fb9d)

### 所感
Goの構造体を定義する書き方とSpringでEntityを定義する書き方が合わさった感じ。
記事の作成者が作成したやり方的には、
- 検証用の Value Object としてユーザー識別子という名前で作成
- 検証ロジックの組み立て
という流れ。

フィールドに対して値オブジェクト内でのロジックで事前検証してると考えられる。
Javaのrecordのコンパクトコンストラクタの要領に近い。

### ソース
```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Ulid,
    pub discriminator: UserDiscriminator,
    pub name: UserName,
    pub email: EmailAddress,
    pub password: Password,
}

impl User {
    pub fn new(
        id: Ulid,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        password: Password,
    ) -> Self {
        Self {
            id,
            discriminator,
            name,
            email,
            password,
        }
    }
}
```