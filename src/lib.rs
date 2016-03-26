#![allow(non_camel_case_types)]


/// Integer type
pub type gint_t = i64;
/// Matrix dimension type
pub type dim_t = gint_t;
/// Stride type
pub type inc_t = gint_t;

// Info functions
extern {
    pub fn bli_info_get_int_type_size_str() -> *const i8;
    pub fn bli_info_get_version_str() -> *const i8;
    pub fn bli_info_get_int_type_size() -> gint_t;
}

/// Error enum.
///
/// Actual C enum has more variants.
#[repr(C)]
pub enum err_t {
    BLIS_SUCCESS = -1,
    BLIS_FAILURE = -2,
    #[doc(hidden)]
    __INCOMPLETE = -140,
}

// Initialization
extern {
    pub fn bli_init() -> err_t;
    pub fn bli_finalize() -> err_t;
    pub fn bli_is_initialized() -> gint_t;
}

const BLIS_TRANS_SHIFT: usize = 3;
const BLIS_CONJ_SHIFT: usize = 4;

/// Conjugation enum
#[repr(C)]
pub enum conj_t {
    BLIS_NO_CONJUGATE = 0,
    BLIS_CONJUGATE = 1 << BLIS_CONJ_SHIFT,
}

pub use self::conj_t::*;

/// Transpose enum
#[repr(C)]
pub enum trans_t {
    BLIS_NO_TRANSPOSE = 0,
    BLIS_TRANSPOSE = 1 << BLIS_TRANS_SHIFT,
    BLIS_CONJ_NO_TRANSPOSE = 1 << BLIS_CONJ_SHIFT,
    BLIS_CONJ_TRANSPOSE = 1 << BLIS_TRANS_SHIFT | 1 << BLIS_CONJ_SHIFT
}

pub use self::trans_t::*;

// Level 1v
extern {
    /// y := y + conjx(x)
    pub fn bli_saddv(conjx: conj_t,
                     m: dim_t,
                     x: *const f32, incx: inc_t,
                     y: *mut f32, incy: inc_t );
    /// y := y + conjx(x)
    pub fn bli_daddv(conjx: conj_t,
                     m: dim_t,
                     x: *const f64, incx: inc_t,
                     y: *mut f64, incy: inc_t );
    /// rho := conjx(x)^T * conjy(y)
    pub fn bli_sdotv(conjx: conj_t,
                     conjy: conj_t,
                     m: dim_t,
                     x: *const f32, incx: inc_t,
                     y: *const f32, incy: inc_t,
                     rho: *mut f32);
    /// rho := conjx(x)^T * conjy(y)
    pub fn bli_ddotv(conjx: conj_t,
                     conjy: conj_t,
                     m: dim_t,
                     x: *const f64, incx: inc_t,
                     y: *const f64, incy: inc_t,
                     rho: *mut f64);
}

// Level 3
extern {
    /// C := beta * C + alpha * transa(A) * transb(B)
    pub fn bli_sgemm(transa: trans_t,
                     transb: trans_t,
                     m: dim_t,
                     n: dim_t,
                     k: dim_t,
                     alpha: *const f32,
                     a: *const f32, rsa: inc_t, csa: inc_t,
                     b: *const f32, rsb: inc_t, csb: inc_t,
                     beta: *const f32,
                     c: *mut f32, rsc: inc_t, csc: inc_t);
    /// C := beta * C + alpha * transa(A) * transb(B)
    pub fn bli_dgemm(transa: trans_t,
                     transb: trans_t,
                     m: dim_t,
                     n: dim_t,
                     k: dim_t,
                     alpha: *const f64,
                     a: *const f64, rsa: inc_t, csa: inc_t,
                     b: *const f64, rsb: inc_t, csb: inc_t,
                     beta: *const f64,
                     c: *mut f64, rsc: inc_t, csc: inc_t);
}

#[test]
fn test() {
    use std::ffi::CStr;
    unsafe {
        let x = bli_info_get_int_type_size_str();
        let s = CStr::from_ptr(x);
        println!("{:?}", s);
        let v = bli_info_get_version_str();
        let s = CStr::from_ptr(v);
        println!("{:?}", s);
    }
}

#[test]
fn sanity_check() {
    use std::mem::size_of;
    unsafe {
        assert_eq!(size_of::<gint_t>() * 8, bli_info_get_int_type_size() as usize);
    }
}

#[test]
fn addv() {
    let mut x = [2.; 16];
    let mut y = [0.; 16];
    for (i, elt) in (&mut x[..]).into_iter().enumerate() {
        *elt = i as f32;
    }
    unsafe {
        bli_init();
        println!("{:?}\n{:?}", x, y);
        bli_saddv(BLIS_NO_CONJUGATE,
                  x.len() as gint_t / 2,
                  x.as_ptr().offset(15), -2,
                  y.as_mut_ptr(), 1);
        println!("{:?}\n{:?}", x, y);
        //assert_eq!(&x, &y);
    }
}

#[test]
fn gemm() {
    let mut a = [2.; 16];
    let b = [1.; 16];
    let mut c = [0.; 16];
    for (i, elt) in (&mut a[..]).into_iter().enumerate() {
        *elt = i as f32;
    }
    let res = [  6.,   6.,   6.,   6.,  22.,  22.,  22.,  22.,  38.,  38.,  38.,
            38.,  54.,  54.,  54.,  54.];
    unsafe {
        bli_init();
        println!("{:?}\n{:?}\n{:?}", a, b, c);
        bli_sgemm(BLIS_NO_TRANSPOSE,
                 BLIS_NO_TRANSPOSE,
                 4, 
                 4,
                 4,
                 &1.,
                 a.as_ptr(), 4, 1,
                 b.as_ptr(), 4, 1,
                 &0.,
                 c.as_mut_ptr(), 4, 1);

        println!("{:?}\n{:?}\n{:?}", a, b, c);
        assert_eq!(&c, &res);
        //assert_eq!(&x, &y);
    }
    unsafe {
        println!("{:?}\n{:?}\n{:?}", a, b, c);
        bli_sgemm(BLIS_NO_TRANSPOSE,
                 BLIS_NO_TRANSPOSE,
                 4, 
                 4,
                 4,
                 &1.,
                 a.as_ptr().offset(12), -4, 1,
                 b.as_ptr(), 4, 1,
                 &0.,
                 c.as_mut_ptr(), 4, 1);

        println!("{:?}\n{:?}\n{:?}", a, b, c);
    }
}
