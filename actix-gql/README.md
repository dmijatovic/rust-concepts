# Actix todo graphQL api

This example is attempt to implement graphQL with actix-web v3. This is new version of actix still in beta.

For graphQL implementation most popular solution seem to be juniper.

## Challenges

I managed to setup basic server, which was simple to do based on documentation, googling on internet and watching some [youtube videos](https://www.youtube.com/watch?v=9q4GcWbAIEM&list=PLRiZb4DNOVQduDWGbqZR3bB4O4D9UDmIQ&index=9&t=0s).

However after using deadpool postgres connection pool to setup async connection there were problems.

To use async/await with juniper at this point in time special git version (and branch) should be included in cargo.toml file

```yml
[dependencies]
juniper = { git = "https://github.com/graphql-rust/juniper", branch = "async-await", features = ["async"] }
```

Unfortunately when including this version of juniper into a project the build with crash due to version coalision of future crate used by juniper and actix-web v3. As both libraries are in beta/instable state at the moment there seem to be not mutch I can do about it now.

I choose to go back to sync juniper and keep v3 actix-web in use.
To solve async calls to postgres I used tokio runtime to await for future to resolve.
