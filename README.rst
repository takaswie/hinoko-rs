================================
Rust bindings for hinoko library
================================

2024/09/21
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhinoko1`` which provides ``Hinoko-1.0.gir``.

  * `<https://git.kernel.org/pub/scm/libs/ieee1394/libhinoko.git/>`_

* The crates are available in `crates.io <https://crates.io/>`_ as well.

The latest release is version 0.7.0.

Crates
======

API bindings for safe and high-level abstractions
-------------------------------------------------

* `hinoko crate <hinoko/README.md>`_

`FFI bindings <https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages>`_
--------------------------------------------------------------------------------------------

* `hinoko-sys crate <hinoko-sys/README.md>`_

License
=======

MIT License

Dependencies
============

* `libhinoko <https://git.kernel.org/pub/scm/libs/ieee1394/libhinoko.git/>`_
* FFI crate (``hinoko-sys``)

  * ``libc`` >= 0.2
  * ``glib-sys`` >= 0.20
  * ``gobject-sys`` >= 0.20
  * ``hinawa-sys`` >= 0.12

* API crate (``hinoko``)

  * ``libc`` >= 0.2
  * ``bitflags`` >= 2.6
  * ``glib`` >= 0.20
  * ``hinawa`` >= 0.12
  * FFI crate (``hinoko-sys``)

Examples
========

See ``hinoko/examples`` directory
