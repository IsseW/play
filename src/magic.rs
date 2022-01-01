pub struct If<const B: bool>;
pub trait True {}
impl True for If<true> {}
pub trait False {}
impl False for If<false> {}

pub struct CTuple<const A: usize, const B: usize>;

pub trait InferEq {}

impl<const N: usize> InferEq for CTuple<N, N> {}

impl<T> InferEq for (T, T) {}
impl<T> InferEq for (T, T, T) {}

const fn within_range(n: usize, start: usize, end: usize) -> bool {
    start < n && n <= end
}

pub const fn log2(n: usize) -> usize {
    std::mem::size_of::<usize>() * 8 - n.leading_zeros() as usize
}

pub const fn bits_to_bytes(n: usize) -> usize {
    (n + 7) / 8
}

pub trait Type {
    type U;
    type I;
    const BITS: usize;
}

mod bts {
    use super::{If, True, Type};

    pub struct Bytes<const N: usize>;
    impl Type for Bytes<1> {
        type U = u8;
        type I = i8;
        const BITS: usize = 8;
    }
    impl Type for Bytes<2> {
        type U = u16;
        type I = i16;
        const BITS: usize = 16;
    }
    impl Type for Bytes<3> {
        type U = u32;
        type I = i32;
        const BITS: usize = 32;
    }
    impl Type for Bytes<4> {
        type U = u64;
        type I = i64;
        const BITS: usize = 64;
    }
    impl Type for Bytes<5> {
        type U = u128;
        type I = i128;
        const BITS: usize = 64;
    }
    //  impl<const N: usize> Type for Bytes<N>
    //  where
    //      If<{ N > 16 }>: True,
    //  {
    //      // TODO: replace with custom type
    //      type U = u128;
    //      type I = u128;
    //      const BITS: usize = 128;
    //  }
}

pub struct Underlying<const N: usize>;

/// # Examples
///
/// ```
/// assert_eq!(Underlying::<1>::BITS, 8);
/// assert_eq!(Underlying::<2>::BITS, 8);
/// assert_eq!(Underlying::<55>::BITS, 64);
/// ```
impl<const N: usize> Type for Underlying<N>
where
    bts::Bytes<{ log2(bits_to_bytes(N)) }>: Type,
{
    type U = <bts::Bytes<{ log2(bits_to_bytes(N)) }> as Type>::U;
    type I = <bts::Bytes<{ log2(bits_to_bytes(N)) }> as Type>::I;
    const BITS: usize = <bts::Bytes<{ log2(bits_to_bytes(N)) }> as Type>::BITS;
}
