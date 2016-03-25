
BLIS_ is a blas-like basic linear algebra package.

.. _BLIS: https://github.com/flame/blis


Configuration Caveats
=====================

**This is a very experimental version and the bindings are extremely incomplete.**

**In fact, only sgemm and dgemm have real bindings.**

+ Uses configuration `auto` from BLIS's configure script, which will try
  to pick the applicable microarchtechture
+ Always uses pthreads.
+ Help is needed to support Windows

Crate Feature Flags
===================

+ `system`
  
  + Just emit that we should link to system dylib `libblis.so`, don't compile

+ `ccache`

  + Use `ccache gcc` as the compiler. This caches compilation between crates
    and debug/release, if you have ccache.
