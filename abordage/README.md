# abordage - the geocaching webapp

abordage is the crate managing things such as caches, user infos etc... It's currently not even in alpha or working yet! (finally a project that i might finish).

## Table of contents

- [abordage - the geocaching webapp](#abordage---the-geocaching-webapp)
  - [Table of contents](#table-of-contents)
  - [Build the sources](#build-the-sources)
  - [License](#license)
  - [TODO](#todo)

## Build the sources

You'll need the latest Rust stable chain (you can get it [here](https://www.rust-lang.org/tools/install)) and [sqlx-cli tool](https://crates.io/crates/sqlx-cli), and the `geo` C dependency.

Then run the following commands in your terminal

```bash
sqlx database setup
cargo build --release
```

## License

This project is licensed under the GPLv3 licence. I cannot guarantee if this software will work or not, so you are the only responsible if this code get your computer blown away.

Also, credits to crates who made possible this webapp : [axum](https://crates.io/crates/axum), [sqlx](https://github.com/launchbadge/sqlx), and [Dotenvy](https://github.com/nystudio107/dotenvy).

## TODO

- [ ] Correct implementations of the Cache and users routes
- [ ] Implement a webui
- [ ] Expose an API
- [ ] Create a docker image to make installation easier
- [ ] Integrate a fediverse-like system.
