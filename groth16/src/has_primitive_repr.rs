use crate::{Proof, VerifyingKey};
use ark_bn254::{G1Affine, G2Affine};
use ark_ec::short_weierstrass::{Affine, Projective, SWCurveConfig};
use ark_ff::{BigInt, Fp, Fp2, Fp2Config, FpConfig, PrimeField};

/// An object which has a representation in terms of primitive objects (which
/// can be efficiently serialized between host and guest).
pub trait HasPrimitiveRepr {
    type Repr;

    fn to_repr(&self) -> Self::Repr;

    fn from_repr(repr: &Self::Repr) -> Self;
}

impl HasPrimitiveRepr for u32 {
    type Repr = Self;
    fn to_repr(&self) -> Self {
        *self
    }
    fn from_repr(repr: &Self) -> Self {
        *repr
    }
}

impl HasPrimitiveRepr for u8 {
    type Repr = Self;
    fn to_repr(&self) -> Self {
        *self
    }
    fn from_repr(repr: &Self) -> Self {
        *repr
    }
}

// TODO: Optimized HasPrimitiveRepr for [u8; 32] (proof id) as packed u32s

impl<P: FpConfig<N>, const N: usize> HasPrimitiveRepr for Fp<P, N> {
    type Repr = [u64; N];

    fn to_repr(&self) -> Self::Repr {
        self.into_bigint().0
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        Self::from_bigint(BigInt(*repr)).unwrap()
    }
}

impl<P: Fp2Config> HasPrimitiveRepr for Fp2<P>
where
    P::Fp: HasPrimitiveRepr,
{
    type Repr = [<P::Fp as HasPrimitiveRepr>::Repr; 2];

    fn to_repr(&self) -> Self::Repr {
        [self.c0.to_repr(), self.c1.to_repr()]
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        Self {
            c0: P::Fp::from_repr(&repr[0]),
            c1: P::Fp::from_repr(&repr[1]),
        }
    }
}

impl<P: SWCurveConfig> HasPrimitiveRepr for Affine<P>
where
    P::BaseField: HasPrimitiveRepr,
{
    type Repr = [<P::BaseField as HasPrimitiveRepr>::Repr; 2];

    fn to_repr(&self) -> Self::Repr {
        assert!(!self.infinity);
        [self.x.to_repr(), self.y.to_repr()]
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        Self {
            x: P::BaseField::from_repr(&repr[0]),
            y: P::BaseField::from_repr(&repr[1]),
            infinity: false,
        }
    }
}

impl<P: SWCurveConfig> HasPrimitiveRepr for Projective<P>
where
    P::BaseField: HasPrimitiveRepr,
{
    type Repr = [<P::BaseField as HasPrimitiveRepr>::Repr; 3];

    fn to_repr(&self) -> Self::Repr {
        [self.x.to_repr(), self.y.to_repr(), self.z.to_repr()]
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        Self {
            x: P::BaseField::from_repr(&repr[0]),
            y: P::BaseField::from_repr(&repr[1]),
            z: P::BaseField::from_repr(&repr[2]),
        }
    }
}

impl<T: HasPrimitiveRepr> HasPrimitiveRepr for Vec<T> {
    type Repr = Vec<T::Repr>;

    fn to_repr(&self) -> Self::Repr {
        self.iter().map(T::to_repr).collect()
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        repr.iter().map(T::from_repr).collect()
    }
}

impl<T: HasPrimitiveRepr, const N: usize> HasPrimitiveRepr for [T; N] {
    type Repr = [T::Repr; N];

    fn to_repr(&self) -> Self::Repr {
        self.each_ref().map(T::to_repr)
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        repr.each_ref().map(T::from_repr)
    }
}

impl HasPrimitiveRepr for Proof {
    type Repr = (
        <G1Affine as HasPrimitiveRepr>::Repr,
        <G2Affine as HasPrimitiveRepr>::Repr,
        <G1Affine as HasPrimitiveRepr>::Repr,
    );
    fn to_repr(&self) -> Self::Repr {
        (
            self.pi_a.to_repr(),
            self.pi_b.to_repr(),
            self.pi_c.to_repr(),
        )
    }
    fn from_repr(repr: &Self::Repr) -> Self {
        Self {
            pi_a: <G1Affine as HasPrimitiveRepr>::from_repr(&repr.0),
            pi_b: <G2Affine as HasPrimitiveRepr>::from_repr(&repr.1),
            pi_c: <G1Affine as HasPrimitiveRepr>::from_repr(&repr.2),
        }
    }
}

impl HasPrimitiveRepr for VerifyingKey {
    type Repr = (
        <G1Affine as HasPrimitiveRepr>::Repr,
        <G2Affine as HasPrimitiveRepr>::Repr,
        <G2Affine as HasPrimitiveRepr>::Repr,
        <G2Affine as HasPrimitiveRepr>::Repr,
        <Vec<G1Affine> as HasPrimitiveRepr>::Repr,
    );
    fn to_repr(&self) -> Self::Repr {
        (
            self.alpha.to_repr(),
            self.beta.to_repr(),
            self.gamma.to_repr(),
            self.delta.to_repr(),
            self.s.to_repr(),
        )
    }
    fn from_repr(repr: &Self::Repr) -> Self {
        Self {
            alpha: <G1Affine as HasPrimitiveRepr>::from_repr(&repr.0),
            beta: <G2Affine as HasPrimitiveRepr>::from_repr(&repr.1),
            gamma: <G2Affine as HasPrimitiveRepr>::from_repr(&repr.2),
            delta: <G2Affine as HasPrimitiveRepr>::from_repr(&repr.3),
            s: <Vec<G1Affine> as HasPrimitiveRepr>::from_repr(&repr.4),
        }
    }
}
