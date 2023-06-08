# atuin sqlite

atuin server but uses sqlite backend.

## Support ⚠️

While I am a maintainer of `atuin`, This is not an officially supported project and it mostly exists because people kept asking for it and
it wasn't too hard to put here.

## Usage

```sh
cargo install --release --git https://github.com/conradludgate/atuin-server-sqlite
export ATUIN_DB_URI=sqlite://path/to/sqlite/file.db
atuin-server-sqlite-unofficial
```

See the atuin self hosting docs for more info on settings.
