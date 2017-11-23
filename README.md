`huniq`
=======

`huniq` writes deduplicated lines from standard input to standard output. It is
differentiated from the standard `uniq` by not requiring sorted input.
Internally, it uses Rust's standard [HashSet] type to accomplish this.

[HashSet]: https://doc.rust-lang.org/std/collections/struct.HashSet.html

Installation
------------

At this time, `huniq` must be compiled manually.

1. [Install Rust](https://www.rust-lang.org/en-US/install.html)
2. `cargo install --git https://gitlab.alexhamlin.co/ahamlinman/huniq.git`

License
-------

MIT (see LICENSE.txt)
