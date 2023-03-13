#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate bellman;
extern crate pairing;
extern crate rand;


use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof};
use pairing::{Engine, Field, PrimeField};
use pairing::bls12_381::{Bls12, Fr};
use self::rand::{thread_rng, Rng};


// proving that I know x such that x^3 + x + 5 == 35
// Generalized: x^3 + x + 5 == out
pub struct CubeDemo<E: Engine> {
    pub x: Option<E::Fr>,
}

impl <E: Engine> Circuit<E> for CubeDemo<E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let x_val = self.x;
        let x = cs.alloc(|| "x", ||{x_val.ok_or(SynthesisError::AssignmentMissing)})?;

        let temp_1_val = x_val.map(|mut e|{e.square(); e});
        let temp_1 = cs.alloc(|| "temp_1", ||{temp_1_val.ok_or(SynthesisError::AssignmentMissing)})?;

        //enforce x*x = temp_1;
        cs.enforce(||"temp_1", |lc| lc+x, |lc| lc+x, |lc| lc+temp_1);

        let x_cube_val = temp_1_val.map(|mut e|{e.mul_assign(&x_val.unwrap()); e});
        let x_cube = cs.alloc(|| "x_cube", ||{x_cube_val.ok_or(SynthesisError::AssignmentMissing)})?;

        //enforce temp_1*x = output;

        cs.enforce(||"output", |lc| lc+temp_1, |lc| lc+x, |lc| lc+ x_cube);


        let out = cs.alloc_input(||"out",||{let mut tmp = x_cube_val.unwrap(); tmp.add_assign(&x_val.unwrap()); tmp.add_assign(&E::Fr::from_str("5").unwrap()); Ok(tmp)})?;

        cs.enforce(|| "out", |lc| lc+x_cube+x+ (E::Fr::from_str("5").unwrap(),CS::one()), |lc| lc + CS::one(), |lc| lc+ out );

        Ok(())
    }
}

#[test]
fn test_cube_proof(){
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let rng = &mut thread_rng();

    println!("Creating parameters...");

    // Create parameters for our circuit
    let params = {
        let c = CubeDemo::<Bls12> {
            x: None
        };

        generate_random_parameters(c, rng).unwrap()
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");

    // Create an instance of circuit
    let c = CubeDemo::<Bls12> {
        x: Fr::from_str("3")
    };

    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();

    assert!(verify_proof(
        &pvk,
        &proof,
        &[Fr::from_str("35").unwrap()]
    ).unwrap());
}