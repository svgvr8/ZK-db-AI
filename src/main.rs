use std::collections::HashMap;

type Scalar = i64;
type Point = i64;

const G: Point = 2;
const P: Scalar = 101;

fn mod_exp(base: Scalar, exponent: Scalar, modulus: Scalar) -> Scalar {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }

    result
}

fn add_points(a: Point, b: Point) -> Point {
    (a + b) % P
}

fn scalar_multiply(point: Point, scalar: Scalar) -> Point {
    mod_exp(point, scalar, P)
}

fn main() {
    // Prover's secret value (x) and public value (X = g^x)
    let secret_value = 12;
    let public_value = scalar_multiply(G, secret_value);

    // Prover generates a random value (r) and computes the commitment (C = g^r)
    let r = 5;
    let commitment = scalar_multiply(G, r);

    // Verifier generates a random challenge (e)
    let challenge = 7;

    // Prover computes the response (z = r + e * x) modulo P-1
    let response = (r + challenge * secret_value) % (P - 1);

    // Verifier computes g^z and X^e * C
    let g_z = scalar_multiply(G, response);
    let x_e_c = add_points(scalar_multiply(public_value, challenge), commitment);

    // If g^z == X^e * C, the proof is valid
    let proof_valid = g_z == x_e_c;

    println!("Proof is valid: {}", proof_valid);
}
