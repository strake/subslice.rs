Documentation
-------------

Fast substring search for strings and byte strings, using the `two-way algorithm`_.

This is the same code as is included in Rust's libstd to “power” ``str::find(&str)``,
but here it is exposed with some improvements:

- ``subslice::SubsliceExt::find(&self, other: &Self) -> Option<usize>``
- ``subslice::SubsliceExt::rfind(&self, other: &Self) -> Option<usize>``

Interesting Links
-----------------

.. _`two-way algorithm`: http://www-igm.univ-mlv.fr/~lecroq/string/node26.html

- Two Way: http://www-igm.univ-mlv.fr/~lecroq/string/node26.html
- Matters Computational: http://www.jjj.de/fxt/#fxtbook

Notes
-----

Consider denying 0/n factorizations, see
http://lists.gnu.org/archive/html/bug-gnulib/2010-06/msg00184.html
