# abordage - the geocaching webapp

abordage is the crate managing things such as caches, user infos etc... It's currently in early alpha and it's not even working yet! (another project that i might finish this time).

This webapp was made with the following crates : [Rocket](https://rocket.rs), [sqlx](https://github.com/launchbadge/sqlx), and [Dotenvy](https://github.com/nystudio107/dotenvy).

## Build the sources

Needed : the latest Rust stable chain (you can get it [here](https://www.rust-lang.org/tools/install)), [sqlx-cli tool](https://crates.io/crates/sqlx-cli) and a Postgres database server running. Then run the following commands in your terminal

```bash
sqlx database setup
cargo build --release
```

## License

This project is licensed under the GPLv3 licence

## TODO

- [ ] Finish implementations of the Cache and users routes
- [ ] Integrate a webui
- [ ] Create a docker-compose to make installation easier
- [ ] Integrate a fediverse-like system.
