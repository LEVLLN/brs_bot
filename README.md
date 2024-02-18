# Bread bot on Rust

## How to run server

It's easy!

```shell
cargo run
```

## Check and format code

```shell
cargo clippy --all-targets --all-features -- -D warnings
```

## How to SQL migrations

### Default operations

You can see default migration process for sqlx: https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md

```shell
# Init database url
DATABASE_URL=postgres://bread_bot:bread_bot@localhost/bread_bot
# Add new migration file
sqlx migrate add {name_of_migration} --database-url $DATABASE_URL
# Run migrations
sqlx migrate run --database-url $DATABASE_URL
```

**Warning:** If your migration schema generated via pg_dump, you need delete row from migration file:

```sql
-- delete this row from migration file
SELECT pg_catalog.set_config('search_path', '', false);
``` 

If this row is existed will raise
Error `error: while executing migrations: error returned from database: relation "_sqlx_migrations" does not exist`;
Solving problem link: [tap](https://github.com/launchbadge/sqlx/issues/640#issuecomment-775540880)

## Fake or fix existed broken migrations

Fake migrations is not ready in sqlx framework.
For run fake migration or fix broken migrations in _sql_migrations table play next steps:

- Create empty file .sql via `sqlx migrate add`
- Run migration with empty file `sqlx migrate run`
- Write new script to created file
- `sqlx migrate info` will write log that different checksum in console. Log looks like:

```shell
applied migration had checksum 0181c39b1e0d326ea12e215253b708421ca3fe050048c3433c8e8df1717191286888d76af856f25a9f90463ba75f9973fa
local migration has checksum   81c39b1e0d326ea12e215253b708421ca3fe050048c3433c8e8df1717191286888d76af856f25a9f90463ba75f9973fa
```

- Copy `local migration checksum` from log
- Open database shell and update migration version for `_sqlx_migrations` table in `checksum` column as bytea:

```sql
update _sqlx_migrations
set checksum = '\x81c39b1e0d326ea12e215253b708421ca3fe050048c3433c8e8df1717191286888d76af856f25a9f90463ba75f9973fa'::bytea
where version = 20240204005259;
```

- Done!

Information about fake migration method in issue of sqlx: https://github.com/launchbadge/sqlx/issues/911

## Run tests

Need DATABASE_URL for tests and setup database user as superuser, because sqlx create temp databases for tests.

Command for execute:

```shell
DATABASE_URL=postgres://bread_bot:bread_bot@localhost/bread_bot cargo test
```

___
Donate to the project: https://boosty.to/levkey/donate 