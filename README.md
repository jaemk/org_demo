# Org Demo
[![Build Status](https://travis-ci.org/jaemk/org_demo.svg?branch=master)](https://travis-ci.org/jaemk/org_demo)

Note, fully compiled and packaged releases are available for 64bit linux and osx.
See [`releases`](https://github.com/jaemk/org_demo/releases)


## Building

> Note, no building is required if you've downloaded a packaged release

**Backend**

- Install [`rust`](https://rustup.rs/)
- Run `cargo build --release`

Or use the build script to generate statically linked binaries (requires `docker` to be installed)

```bash
./build.py server
```

**Frontend**

Use the build script to build the react frontend and copy files into place (requires `yarn` to be installed)

```bash
./build.py web
```

## Running

Note, the server must be run from the root of the `org_demo` project directory so static files can be found

```bash
# setup database and run migrations
bin/org_demo database migrate

# start the server
# see `org_demo serve --help`
bin/org_demo serve

# Or if you built from source
cargo run -- database migrate
cargo run -- serve
```

