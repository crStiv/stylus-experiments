import json
import subprocess

contract_address = "0x4e5b65fb12d4165e22f5861d97a33ba45c006114"
private_key = "0xdc04c5399f82306ec4b4d654a342f40e2e0620fe39950d967e1e574b32d4dd36"

with open('merkle_proof_test_data.json', 'r') as f:
    data = json.load(f)

proof = data["proof"]
height = data["header"]["fields"]["height"]
header_commitment = data["header_commitment"]
block_merkle_root = data["block_merkle_root"]
hotshot_commitment = data["hotshot_commitment"]

proof = list(json.dumps(proof).encode("utf-8"))
block_merkle_root = list(str(block_merkle_root).encode('utf-8'))

call_merkle_proof_command = [
    "cast", "call", contract_address,
    "verifyMerkleProof(uint8[],uint8[],uint8[],uint64, uint8[])",
    str(proof),
    str(block_merkle_root),
    str(hotshot_commitment),
    str(height),
    str(header_commitment),
    "--private-key", private_key,
    "--rpc-url", "http://localhost:8547",
]

subprocess.run(call_merkle_proof_command)

call_merkle_proof_command = [
    "cast", "estimate", contract_address,
    "verifyMerkleProof(uint8[],uint8[],uint8[],uint64, uint8[])",
    str(proof),
    str(block_merkle_root),
    str(hotshot_commitment),
    str(height),
    str(header_commitment),
    "--rpc-url", "http://localhost:8547",
]

subprocess.run(call_merkle_proof_command)
