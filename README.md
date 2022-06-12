# Introduction

**ngxcache** is a simple tool to display metadata from Nginx cache files.

# Table of contents

* [Usage](#usage)
* [Build](#build)

## Usage

-a, --ascending sort the output in ascending order, this parameter has no effect unless used with *-o (--order-by)*.
-d, --descending sort the output in descending order; this parameter has no effect unless used with *-o (--order-by)*.
-h, --help Displays help information and exit.
-o, --order-by Specifies the property to sort the output by. This can be cached, expired or modified dates, filename or key.
-p, --path path to the location of Nginx cache files. Path can also be passed as the last argument when running **ngxcache**.
-v, --version Print version information

It is also possible to set the following environment variables to set a default behaviour, for example setting the NGXCACHE_PATH to allow **ngxcache** to be executed without any arguments. If both argument and environment variable are defined the argument will take precedente.

*NGXCACHE_PATH* path to the location to the Nginx cache files.
*NGXCACHE_ORDER_BY* Property to sort the output by, see --order-by argument
*NGXCACHE_ORDER* This can be *ascending* or *descending*.

```bash
# Proceses the cache files in /dev/shm/nginx ordered by modified date in ascending order.
ngxcache -o modified -a /dev/shm/nginx
```

```bash
# Similar to the previous example but using the path defined in the environment.
export NGXCACHE_PATH=/dev/shm/nginx
ngxcache -o modified -a
```

## Build

```bash
cargo build --release
```

**ngxcache** has a dependency on LIBC, to have Rust to static link LIBC, *musl* target needs to used on Linux

```
rustup target add x86_64-unknown-linux-musl
cargo build --target=x86_64-unknown-linux-musl --release
```