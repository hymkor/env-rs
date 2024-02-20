env.exe for Windows
===================

```
$ env {options} {NAME=VALUE}...  COMMAND ARGS...
```

+ `-v`, `--debug`
    + print verbose information
+ `--version`
    + output version information
+ `-h`, `--help`
    + display help

Install
-------

Download the binary package from [Releases](https://github.com/hymkor/env-rs/releases) and extract the executable.

### for scoop-installer

```
scoop install https://raw.githubusercontent.com/hymkor/env-rs/master/env-rs.json
```

or

```
scoop bucket add  https://github.com/hymkor/scoop-bucket
scoop install env-rs
```
