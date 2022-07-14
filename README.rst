================================
Rust bindings for hinoko library
================================

2022/07/14
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhinoko0`` which provides ``Hinoko-0.0.gir``.

  * `<https://github.com/takaswie/libhinoko>`_

* The crates are available in `crates.io <https://crates.io/>`_ as well.

The latest release is version 0.1.0.

Crates
======

API bindings for safe and high-level abstractions
-------------------------------------------------

* [hinoko crate](hinoko/README.md)

`FFI bindings <https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages>`_
--------------------------------------------------------------------------------------------

* [hinoko-sys crate](hinoko-sys/README.md)

License
=======

MIT License

Dependencies
============

* `libhinoko <https://github.com/takaswie/libhinoko>`_
* FFI crate (``hinoko-sys``)

  * ``libc`` >= 0.2
  * ``glib-sys`` >= 0.15
  * ``gobject-sys`` >= 0.15

* API crate (``hinoko``)

  * ``libc`` >= 0.2
  * ``bitflags`` >= 1.0
  * ``glib`` >= 0.15
  * FFI crate (``hinoko-sys``)

Examples
========

See ``hinoko/examples`` directory
