// Welcome to the gnark playground!
package main

import (
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
)

// gnark is a zk-SNARK library written in Go. Circuits are regular structs.
// The inputs must be of type frontend.Variable and make up the witness.
// The witness has a
//   - secret part --> known to the prover only
//   - public part --> known to the prover and the verifier
type Circuit struct {
	X frontend.Variable `gnark:"x"`       // x  --> secret visibility (default)
	Y frontend.Variable `gnark:",public"` // Y  --> public visibility
}

// Define declares the circuit logic. The compiler then produces a list of constraints
// which must be satisfied (valid witness) in order to create a valid zk-SNARK
func (circuit *Circuit) Define(api frontend.API) error {
	// compute x**3 and store it in the local variable x3.
	x3 := api.Mul(circuit.X, circuit.X, circuit.X)

	// compute x**3 + x + 5 and store it in the local variable res
	res := api.Add(x3, circuit.X, 5)

	// assert that the statement x**3 + x + 5 == y is true.ß
	api.AssertIsEqual(circuit.Y, res)
	return nil
}

func main() {
	var cubeCircuit Circuit
	// compile circuit to r1cs
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &cubeCircuit)

	if err != nil {
		return
	}
	//Groth
	// One time setup
	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		return
	}
	// -- witness.json --
	// {
	//     "x": 3,
	//     "Y": 35
	// }
	assignment := &Circuit{
		X: 3,
		Y: 35,
	}
	witness, _ := frontend.NewWitness(assignment, ecc.BN254.ScalarField())
	proof, err := groth16.Prove(r1cs, pk, witness)
	if err != nil {
		return
	}
	pubAssignment := &Circuit{
		X: 0,
		Y: 35,
	}
	publicWitness, _ := frontend.NewWitness(pubAssignment, ecc.BN254.ScalarField())
	groth16.Verify(proof, vk, publicWitness)

	// 3. Write solidity smart contract into a fileß
	f, _ := os.Create("verify_solidity.sol")
	vk.ExportSolidity(f)
}
