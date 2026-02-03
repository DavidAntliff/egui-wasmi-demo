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

## Pixel Buffer Handling

In some cases, a guest may need a dedicated pixel buffer to write to. In other cases, the guest may have its own buffers
that it wishes the host to read from. So we support both scenarios:

* Host-provided memory for the guest to use as a pixel-buffer,
* Guest-provided memory for the host to render without copying.

Host-provided memory for the guest to use as a pixel-buffer (writing):

* Guest defines required memory with some initial size,
* Host calls `memory.grow()` to allocate additional memory,
* This new region is used as a pixel buffer, guaranteed to be unused by the guest,
* Host passes this offset to the guest to use as a pixel buffer as parameters of `update()`.

Guest-provided memory for the host to use as a pixel-buffer (rendering)

* Guest defines required memory with some initial size,
* Guest writes pixel data into this memory region,
* Host reads this memory region to get pixel data for rendering
* Guest provides offset and size of pixel buffer to host as return value of `update()`.

The guest can return either the offset of its own pixel buffer, or the offset of some static buffer in the guest,
or the offset of the host-provided pixel buffer.

This allows a guest to either use a host-provided pixel buffer, or provide its own pixel buffer for the host to read
from, depending on its needs. This allows for interpreter-based WASM guests to use host-provided memory, instead of
requiring interpreter memory to be pinned for the host to safely read from, as well as allowing for zero-copy rendering
when the guest has its own pixel buffers.

## Roadmap

TODO
