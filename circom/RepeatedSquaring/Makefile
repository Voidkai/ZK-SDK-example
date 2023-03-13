SHELL = zsh

circom = RepeatedSquaring.circom
r1cs = RepeatedSquaring.r1cs
wasm = RepeatedSquaring_js/RepeatedSquaring.wasm
wit_gen = RepeatedSquaring_js/generate_witness.js
compile_outputs = RepeatedSquaring_js/witness_calculator.js $(r1cs) $(wasm) $(wit_gen)
pk = RepeatedSquaring.pk
vk = RepeatedSquaring.vk
ptau = RepeatedSquaring.ptau
keys = $(pk) $(vk)
p_input = RepeatedSquaring.input.json
wit = RepeatedSquaring.wtns
pf = RepeatedSquaring.pf.json
inst = RepeatedSquaring.inst.json
prove_outputs = $(pf) $(inst)

all: verify

$(compile_outputs): $(circom)
	circom $< --r1cs --wasm

$(ptau):
	snarkjs powersoftau new bn128 12 tmp.ptau
	snarkjs powersoftau prepare phase2 tmp.ptau $(ptau)
	rm tmp.ptau

$(keys): $(ptau) $(r1cs)
	snarkjs groth16 setup $(r1cs) $(ptau) $(pk)
	snarkjs zkey export verificationkey $(pk) $(vk)

$(wit): $(p_input) $(wasm) $(wit_gen)
	node $(wit_gen) $(wasm) $(p_input) $@

$(prove_outputs): $(wit) $(pk)
	snarkjs groth16 prove $(pk) $(wit) $(pf) $(inst)

.PHONY = verify clean

verify: $(pf) $(inst) $(vk)
	snarkjs groth16 verify $(vk) $(inst) $(pf)

clean:
	rm -f $(compile_outputs) $(ptau) $(keys) $(wit) $(prove_outputs)
	rmdir RepeatedSquaring_js

