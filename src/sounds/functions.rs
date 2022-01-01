use crate::magic::{CTuple, InferEq};

pub struct Negate<F: Func>(F);

impl<F: Func> Func for Negate<F> {
    fn sample(&mut self, t: f64) -> f64 {
        -self.0.sample(t)
    }
}

pub struct Blend<A: Func, B: Func> {
    t: f64,
    a: A,
    b: B,
}

impl<A: Func, B: Func> Func for Blend<A, B> {
    fn sample(&mut self, t: f64) -> f64 {
        self.a.sample(t) * (1.0 - self.t) + self.b.sample(t) * self.t
    }
}

pub struct Add<A: Func, B: Func> {
    a: A,
    b: B,
}

impl<A: Func, B: Func> Func for Add<A, B> {
    fn sample(&mut self, t: f64) -> f64 {
        self.a.sample(t) + self.b.sample(t)
    }
}
pub struct Sub<A: Func, B: Func> {
    a: A,
    b: B,
}

impl<A: Func, B: Func> Func for Sub<A, B> {
    fn sample(&mut self, t: f64) -> f64 {
        self.a.sample(t) - self.b.sample(t)
    }
}

pub struct Mul<A: Func, B: Func> {
    a: A,
    b: B,
}

impl<A: Func, B: Func> Func for Mul<A, B> {
    fn sample(&mut self, t: f64) -> f64 {
        self.a.sample(t) * self.b.sample(t)
    }
}

pub struct Div<A: Func, B: Func> {
    a: A,
    b: B,
}

impl<A: Func, B: Func> Func for Div<A, B> {
    fn sample(&mut self, t: f64) -> f64 {
        self.a.sample(t) / self.b.sample(t)
    }
}

pub struct Chain<A: Func, B: Func> {
    a: A,
    b: B,
}

impl<A: Func, B: Func> Func for Chain<A, B> {
    fn sample(&mut self, t: f64) -> f64 {
        self.a.sample(self.b.sample(t))
    }
}

pub fn pow<const F: f64>(t: f64) -> f64 {
    t.powf(F)
}

impl<F: FnMut(f64) -> f64> Func for F {
    fn sample(&mut self, t: f64) -> f64 {
        self(t)
    }
}

impl Func for f64 {
    fn sample(&mut self, _: f64) -> f64 {
        *self
    }
}

pub struct Mix<F: Func, S: Sampler<N>, B: Sampler<N>, const N: usize> {
    func: F,
    sampler: S,
    blender: B,
}

impl<F: Func, S: Sampler<N>, B: Sampler<N>, const N: usize> Func for Mix<F, S, B, N> {
    fn sample(&mut self, t: f64) -> f64 {
        (0..N)
            .map(|i| self.func.sample(t / self.sampler.blend_at(i)) / self.blender.blend_at(i))
            .sum::<f64>()
    }
}

pub trait Func: Sized {
    fn sample(&mut self, t: f64) -> f64;

    fn chain<F: Func>(self, other: F) -> Chain<Self, F> {
        Chain { a: self, b: other }
    }

    fn blend<F: Func>(self, other: F, t: f64) -> Blend<Self, F> {
        Blend {
            t,
            a: self,
            b: other,
        }
    }

    fn add<F: Func>(self, other: F) -> Add<Self, F> {
        Add { a: self, b: other }
    }

    fn sub<F: Func>(self, other: F) -> Sub<Self, F> {
        Sub { a: self, b: other }
    }

    fn mul<F: Func>(self, other: F) -> Mul<Self, F> {
        Mul { a: self, b: other }
    }

    fn div<F: Func>(self, other: F) -> Div<Self, F> {
        Div { a: self, b: other }
    }

    fn neg(self) -> Negate<Self> {
        Negate(self)
    }

    fn mix<const N: usize, S: Sampler<N>, B: Sampler<N>>(
        self,
        sampler: S,
        blend: B,
    ) -> Mix<Self, S, B, N> {
        Mix {
            func: self,
            sampler,
            blender: blend,
        }
    }
}

pub trait Sampler<const SAMPLES: usize> {
    fn blend_at(&self, i: usize) -> f64;
}

impl<const SAMPLES: usize, F: Fn(usize) -> f64> Sampler<SAMPLES> for F {
    fn blend_at(&self, i: usize) -> f64 {
        (self)(i)
    }
}

impl<const N: usize, const SAMPLES: usize> Sampler<SAMPLES> for [f64; N]
where
    CTuple<N, SAMPLES>: InferEq,
{
    fn blend_at(&self, i: usize) -> f64 {
        self[i]
    }
}

impl<const SAMPLES: usize> Sampler<SAMPLES> for () {
    fn blend_at(&self, _: usize) -> f64 {
        1.0
    }
}

impl<const SAMPLES: usize> Sampler<SAMPLES> for f64 {
    fn blend_at(&self, i: usize) -> f64 {
        1.0 + self * i as f64
    }
}

pub fn pow_sampler<const F: f64>(i: usize) -> f64 {
    F.powi(i as i32)
}

pub fn saw(t: f64) -> f64 {
    t.fract() * 2.0 - 1.0
}

pub fn tri(t: f64) -> f64 {
    (t.fract() - 0.5).abs() * 4.0 - 1.0
}

pub fn sin(t: f64) -> f64 {
    t.sin()
}

pub fn lin<const F: f64>(t: f64) -> f64 {
    t * F
}
