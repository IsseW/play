use bitvec::prelude::*;
use byteorder::BigEndian;
use std::{
    io::Write,
    marker::PhantomData,
    mem,
    ops::{Add, Deref},
};

use bitvec::slice::BitSlice;

use crate::magic::{bits_to_bytes, Type, Underlying};

pub struct U<const N: usize>;

type BSlice = BitSlice<Lsb0, u8>;

pub trait BitType {
    type Underlying: Sized;
    const BITS: usize;

    // fn construct(value: Self::Underlying, slice: &mut BSlice)
    // where
    //     [u8; mem::size_of::<Self::Underlying>()]: Sized,
    // {
    //     let t = unsafe { mem::transmute::<_, [u8; mem::size_of::<Self::Underlying>()]>(value) };
    //     slice.copy_from_bitslice(t.view_bits())
    // }

    //fn get_aligned<T: BitType>(self, bits: &Bits::<T>, start: usize) -> Self::Underlying {
    //    let value = mem::size_of::<Self::Underlying>();
    //}
}

impl<const N: usize> BitType for U<N>
where
    Underlying<N>: Type,
{
    type Underlying = <Underlying<N> as Type>::U;
    const BITS: usize = N;
}

impl<A: BitType, B: BitType> BitType for (A, B) {
    type Underlying = (A::Underlying, B::Underlying);
    const BITS: usize = A::BITS + B::BITS;
}

pub struct Bits<T: BitType>([u8; bits_to_bytes(T::BITS)])
where
    [(); bits_to_bytes(T::BITS)]: Default;

impl<T: BitType> Bits<T>
where
    [(); bits_to_bytes(T::BITS)]: Default,
{
    pub fn new() -> Self {
        Self([0; bits_to_bytes(T::BITS)])
    }
}
