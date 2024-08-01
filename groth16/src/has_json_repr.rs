use crate::{Proof, VerifyingKey};
use anyhow::Error;
use anyhow::Result;
use ark_bn254::{G1Affine, G2Affine};
use ark_ec::models::short_weierstrass::Affine;
use ark_ff::{BigInt, Fp, Fp2, Fp2Config, FpConfig};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::array::TryFromSliceError;
use std::fs::File;
use std::str::FromStr;

/// Types with a Json representation
pub trait HasJsonRepr: Sized {
    type JsonRepr: DeserializeOwned + Serialize;

    fn to_json(&self) -> Self::JsonRepr;

    fn from_json(repr: &Self::JsonRepr) -> Result<Self>;
}

pub fn load_json<T>(path: &str) -> T
where
    T: HasJsonRepr,
    T::JsonRepr: DeserializeOwned,
{
    HasJsonRepr::from_json(&serde_json::from_reader(File::open(path).unwrap()).unwrap()).unwrap()
}

impl<T: HasJsonRepr> HasJsonRepr for Vec<T> {
    type JsonRepr = Vec<T::JsonRepr>;

    fn to_json(&self) -> Self::JsonRepr {
        self.iter().map(T::to_json).collect()
    }

    fn from_json(repr: &Self::JsonRepr) -> Result<Self> {
        repr.iter().map(T::from_json).collect::<Result<Vec<_>, _>>()
    }
}

fn u64_from_le_bytes(bytes: &[u8; 8]) -> u64 {
    (bytes[0] as u64)
        | ((bytes[1] as u64) << 8)
        | ((bytes[2] as u64) << 16)
        | ((bytes[3] as u64) << 24)
        | ((bytes[4] as u64) << 32)
        | ((bytes[5] as u64) << 40)
        | ((bytes[6] as u64) << 48)
        | ((bytes[7] as u64) << 56)
}

fn u64_from_le_bytes_slice(bytes: &[u8]) -> Result<u64> {
    let bytes: Result<[u8; 8]> = bytes
        .try_into()
        .map_err(|e| Error::msg(format!("try from slice: {e}")));
    Ok(u64_from_le_bytes(&bytes?))
}

fn le_bytes32_from_hex(s: &str) -> Result<[u8; 32], String> {
    fn from_sanitized_hex(s: &str) -> Result<[u8; 32], String> {
        let hex_bytes = hex::decode(s).unwrap_or_else(|e| panic!("invalid hex: {e}"));
        let num_bytes = hex_bytes.len();
        let byte_offset = 32 - num_bytes;
        let mut bytes = [0u8; 32];
        bytes[byte_offset..].clone_from_slice(&hex_bytes);
        bytes.reverse();
        Ok(bytes)
    }

    // Remove the leading 0x
    let s = if let Some(stripped) = s.strip_prefix("0x") {
        stripped
    } else {
        s
    };

    if s.len() % 2 == 0 {
        from_sanitized_hex(s)
    } else {
        let mut new_s = String::with_capacity(s.len() + 1);
        new_s.push('0');
        new_s.push_str(s);
        assert!(new_s.len() % 2 == 0);
        from_sanitized_hex(&new_s)
    }
}

impl<P: FpConfig<4>> HasJsonRepr for Fp<P, 4>
where
    Fp<P, 4>: FromStr,
{
    type JsonRepr = String;

    fn to_json(&self) -> Self::JsonRepr {
        self.to_string()
    }

    fn from_json(repr: &Self::JsonRepr) -> Result<Self> {
        if repr.starts_with("0x") {
            let bytes = le_bytes32_from_hex(repr)
                .map_err(|e| Error::msg(format!("Failed to parse Fr hex string: {repr}: {e}")))?;
            let b: Result<BigInt<4>, TryFromSliceError> = {
                Ok(BigInt::<4>([
                    u64_from_le_bytes_slice(&bytes[0..8])?,
                    u64_from_le_bytes_slice(&bytes[8..16])?,
                    u64_from_le_bytes_slice(&bytes[16..24])?,
                    u64_from_le_bytes_slice(&bytes[24..32])?,
                ]))
            };
            let b = b.map_err(|e| Error::msg(format!("try from slice: {e}")))?;
            P::from_bigint(b).ok_or_else(|| Error::msg("failed converting to bigint"))
        } else {
            Self::from_str(repr).map_err(|_| Error::msg("Failed to parse Fr hex string: {repr}"))
        }
    }
}

impl HasJsonRepr for Affine<ark_bn254::g1::Config> {
    type JsonRepr = G1AffineJson;
    fn to_json(&self) -> Self::JsonRepr {
        assert!(!self.infinity);
        [self.x.to_json(), self.y.to_json()]
    }
    fn from_json(repr: &Self::JsonRepr) -> Result<Self> {
        Ok(Self {
            x: ark_bn254::Fq::from_json(&repr[0])?,
            y: ark_bn254::Fq::from_json(&repr[1])?,
            infinity: false,
        })
    }
}

pub type Fp2Json = [String; 2];

impl<P: Fp2Config> HasJsonRepr for Fp2<P>
where
    P::Fp: HasJsonRepr<JsonRepr = String>,
{
    type JsonRepr = Fp2Json;

    fn to_json(&self) -> Self::JsonRepr {
        [self.c0.to_json(), self.c1.to_json()]
    }

    fn from_json(repr: &Self::JsonRepr) -> Result<Self> {
        Ok(Self {
            c0: P::Fp::from_json(&repr[0])?,
            c1: P::Fp::from_json(&repr[1])?,
        })
    }
}

impl HasJsonRepr for Affine<ark_bn254::g2::Config> {
    type JsonRepr = G2AffineJson;
    fn to_json(&self) -> Self::JsonRepr {
        assert!(!self.infinity);
        [self.x.to_json(), self.y.to_json()]
    }
    fn from_json(repr: &Self::JsonRepr) -> Result<Self> {
        Ok(Self {
            x: ark_bn254::Fq2::from_json(&repr[0])?,
            y: ark_bn254::Fq2::from_json(&repr[1])?,
            infinity: false,
        })
    }
}

pub type G1AffineJson = [String; 2];
pub type G2AffineJson = [[String; 2]; 2];

#[derive(Serialize, Deserialize)]
pub struct VerifyingKeyJson {
    pub alpha: G1AffineJson,
    pub beta: G2AffineJson,
    pub gamma: G2AffineJson,
    pub delta: G2AffineJson,
    pub s: Vec<G1AffineJson>,
}

impl HasJsonRepr for VerifyingKey {
    type JsonRepr = VerifyingKeyJson;
    fn to_json(&self) -> Self::JsonRepr {
        Self::JsonRepr {
            alpha: self.alpha.to_json(),
            beta: self.beta.to_json(),
            gamma: self.gamma.to_json(),
            delta: self.delta.to_json(),
            s: self.s.to_json(),
        }
    }
    fn from_json(json: &Self::JsonRepr) -> Result<Self> {
        Ok(Self {
            alpha: G1Affine::from_json(&json.alpha)?,
            beta: G2Affine::from_json(&json.beta)?,
            gamma: G2Affine::from_json(&json.gamma)?,
            delta: G2Affine::from_json(&json.delta)?,
            s: Vec::<G1Affine>::from_json(&json.s)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProofJson {
    pub pi_a: G1AffineJson,
    pub pi_b: G2AffineJson,
    pub pi_c: G1AffineJson,
}

impl HasJsonRepr for Proof {
    type JsonRepr = ProofJson;
    fn to_json(&self) -> Self::JsonRepr {
        Self::JsonRepr {
            pi_a: self.pi_a.to_json(),
            pi_b: self.pi_b.to_json(),
            pi_c: self.pi_c.to_json(),
        }
    }
    fn from_json(json: &Self::JsonRepr) -> Result<Self> {
        Ok(Self {
            pi_a: G1Affine::from_json(&json.pi_a)?,
            pi_b: G2Affine::from_json(&json.pi_b)?,
            pi_c: G1Affine::from_json(&json.pi_c)?,
        })
    }
}
