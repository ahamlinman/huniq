`huniq`
=======

`huniq` writes deduplicated lines from standard input to standard output. It is
differentiated from the standard `uniq` by not requiring sorted input.
Internally, it uses Rust's standard [HashSet] type to accomplish this.

[HashSet]: https://doc.rust-lang.org/std/collections/struct.HashSet.html

Why does this exist?
--------------------

I originally created `huniq` for use in a pipeline with [fzf]. Since `huniq`
can start printing lines immediately (whereas `sort -u` needs to read all input
first), fzf can start matching more quickly.

[fzf]: https://github.com/junegunn/fzf

Plus, it was a good excuse to finally start learning Rust. (This is the real
reason, since I'm sure that better versions of this idea are already
available.)

Installation
------------

At this time, `huniq` must be compiled manually.

1. [Install Rust](https://www.rust-lang.org/en-US/install.html)
2. `cargo install --git https://github.com/ahamlinman/huniq.git`

License
-------

MIT (see LICENSE.txt)
