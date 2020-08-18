# SQLX todo Postgres data handling

## Dependencies

After ading sqlx library cargo check crashed with openssl errors. I found the [solution on github](https://github.com/launchbadge/sqlx/issues/473). I needed to include following openssl dependecy in the cargo.toml file and the error was gone.

```toml
[dependencies.openssl]
version = "0.10.29"
features = [
    "vendored"
]
```
