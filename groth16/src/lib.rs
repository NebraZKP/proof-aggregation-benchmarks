use anyhow::{Error, Result};
use ark_bn254::{Bn254, Fq12, Fr, G1Affine, G1Projective, G2Affine};
use ark_ec::pairing::Pairing;
use ark_ff::Field;

pub mod has_json_repr;
pub mod has_primitive_repr;

/// Public inputs
pub type Inputs = Vec<Fr>;

#[derive(Debug, PartialEq)]
pub struct VerifyingKey {
    pub alpha: G1Affine,
    pub beta: G2Affine,
    pub gamma: G2Affine,
    pub delta: G2Affine,
    pub s: Vec<G1Affine>,
}

#[derive(Debug, PartialEq)]
pub struct Proof {
    pub pi_a: G1Affine,
    pub pi_b: G2Affine,
    pub pi_c: G1Affine,
}

/// Groth16 verifier
pub fn verify(vk: &VerifyingKey, proof: &Proof, inputs: &Inputs) -> Result<()> {
    // Check:
    //   e(-pf.a, pf.b)
    //   e(vk.alpha, vk.beta)
    //   e(p, vk.gamma)
    //   e(pf.c, vk.delta)
    //   == Gt(1)
    //
    // where:
    //   p = vk.s[0] + \sum_i=1^\ell input[i] * vk.s[i]

    // Naive computation of P
    let num_inputs = inputs.len();
    let mut p: G1Projective = vk.s[0].into();
    for (i, input) in inputs.iter().enumerate().take(num_inputs) {
        p += vk.s[i + 1] * input
    }

    let miller_out = Bn254::multi_miller_loop(
        [-proof.pi_a, vk.alpha, p.into(), proof.pi_c],
        [proof.pi_b, vk.beta, vk.gamma, vk.delta],
    );
    let pairing_result = Bn254::final_exponentiation(miller_out);
    if let Some(result) = pairing_result {
        if result.0 == Fq12::ONE {
            Ok(())
        } else {
            Err(Error::msg("pairing result"))
        }
    } else {
        Err(Error::msg("pairing failed"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::has_json_repr::load_json;
    use ark_ff::One;

    #[test]
    fn test_groth16() {
        let proof: Proof = load_json("src/data/proof.json");
        let vk: VerifyingKey = load_json("src/data/vk.json");
        let mut inputs: Inputs = load_json("src/data/inputs.json");

        assert!(verify(&vk, &proof, &inputs).is_ok());

        // Failure case
        inputs[0] = Fr::one();
        assert!(verify(&vk, &proof, &inputs).is_err())
    }
}
