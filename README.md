cap
===

Quickly capture stream-of-thought text to a timestamped sqlite database.

[![CircleCI](https://circleci.com/gh/ckampfe/cap.svg?style=svg)](https://circleci.com/gh/ckampfe/cap)

## use

```
$ cap "some text you want to remember"
# or:
$ echo "some text you want to remember" | cap
# or:
$ some_program_with_useful_output | cap
```

This inserts the text into a sqlite database that by default lives at `~/cap.db`.
You can make your own database in any other location, and then alias `cap` to specify
your custom location like `cap -l /some/other/location/cap.db`. The name of the db is
not significant, and is only a default.


You can then view your data with [cap_viewer](https://github.com/ckampfe/cap_viewer) (a simple webapp) or
query it like:

```
$ sqlite3 ~/cap.db
sqlite> SELECT * FROM entries
```


The schema of the entries table is:

```
CREATE TABLE IF NOT EXISTS entries (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  body TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
```

## install

```
$ brew install sqlite
$ git clone git@github.com:ckampfe/cap.git
$ cd cap
$ cargo install --path . --force
```

## help

```
clark$> cap -h
cap 0.1.0
Clark Kampfe <clark.kampfe@gmail.com>

USAGE:
    cap [OPTIONS] [entry]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --database-location <db_location>

ARGS:
    <entry>
```
