# Running Migrator CLI

接続情報は Secret Manager の `daily-log-db`（JSON）から読みます。実行前に Application Default Credentials を用意し、`daily-log/.env` に次を書いてください。値の例は `daily-log/.env.example` にあります。

- `GCP_PROJECT_ID`
- `DB_SECRET_ID`
- `DB_HOST`（Cloud SQL Auth Proxy なら `127.0.0.1`。未設定時も `127.0.0.1`）

```sh
gcloud auth application-default login
```

パスワード、DB 名、スキーマはシークレット側にあります。Terraform の tfvars は参照しません。

- Generate a new migration file
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```
