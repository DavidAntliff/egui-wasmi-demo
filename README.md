# egui-wasmi-demo

[![dependency status](https://deps.rs/repo/github/davidantliff/egui-wasmi-demo/status.svg)](https://deps.rs/repo/github/davidantliff/egui-wasmi-demo)
[![Build Status](https://github.com/davidantliff/egui-wasmi-demo/workflows/CI/badge.svg)](https://github.com/davidantliff/egui-wasmi-demo/actions?workflow=CI)

[Online Demo](https://davidantliff.github.io/egui-wasmi-demo/)

## Building

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Native Locally

To build and run natively:

```
 $ just run
```

### Web Locally

To build and run as a WASM application in a local web server:

```
 $ just serve
```

Then visit [http://127.0.0.1:8080/index.html#dev](http://127.0.0.1:8080/index.html#dev).

