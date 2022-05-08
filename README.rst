====================
Hinoko Rust bindings
====================

2022/05/08
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhinoko0`` which provides ``Hinoko-0.0.gir``.

  * `<https://github.com/takaswie/libhinoko>`_

License
=======

MIT License

Dependencies
============

* Rust version 1.57 or later (edition 2021)
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

How to generate FFI and API crates
==================================

::

    $ ./generator.py

end
