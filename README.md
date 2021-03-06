# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Kos... 

Just another crate for ANSI.

[![CI Pipeline](https://github.com/veminovici/kos/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/kos/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/veminovici/kos/branch/master/graph/badge.svg?token=bp8S7RfQld)](https://codecov.io/gh/veminovici/kos)
[![Coverage Status](https://coveralls.io/repos/github/veminovici/kos/badge.svg)](https://coveralls.io/github/veminovici/kos)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/kos)](https://github.com/veminovici/kos)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/kos)](https://github.com/veminovici/kos)

[![Github Actions](https://buildstats.info/github/chart/veminovici/kos)](https://github.com/veminovici/kos)

</br>

### Example

```rust
use kos::*;

println!(
    "display: Here is a '{}' example",
    ITALIC
        .to_ansi("hello world"));

println!(
    "debug: Here is a '{}' example",
    RED.bg(YELLOW)
        .strikethrough()
        .to_ansi("Hello world! (red, yellow, and strikethrough)")
);
```

### A *thank you* note !!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.
