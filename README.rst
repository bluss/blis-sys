
BLIS_ is a blas-like basic linear algebra package.

.. _BLIS: https://github.com/flame/blis


Configuration Caveats
=====================

**This is a very experimental version and the bindings are extremely incomplete.**

**In fact, only sgemm and dgemm have real bindings.**

+ Uses configuration `auto` from BLIS's configure script, which will try
  to pick the applicable microarchtechture
+ Always uses pthreads.
+ Compiler flags are set to optimization, but not tuned
+ BLIS is far from complete, unimplemented microkernels fall back to
  the reference implementation.
+ `BLIS API Quick Reference`__
+ Help is needed to support Windows

__ https://github.com/flame/blis/wiki/BLISAPIQuickReference

Crate Feature Flags
===================

+ `system`
  
  + Just emit that we should link to system dylib `libblis.so`, don't compile

+ `ccache`

  + Use `ccache gcc` as the compiler. This caches compilation between crates
    and debug/release, if you have ccache.
