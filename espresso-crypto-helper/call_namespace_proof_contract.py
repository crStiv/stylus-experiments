import json
import subprocess

with open('namespace_proof_test_data.json', 'r') as f:
    data = json.load(f)

namespace = data["namespace"]
proof_bytes = data["ns_proof"]
commit_bytes = data["vid_commit"]
ns_table_bytes = data["ns_table"]
tx_comm_bytes = data["tx_commit"]
common_data_bytes = data["vid_common"]

contract_address = "0x4e5b65fb12d4165e22f5861d97a33ba45c006114"
private_key = "0xdc04c5399f82306ec4b4d654a342f40e2e0620fe39950d967e1e574b32d4dd36"

call_namespace_proof_command = [
    "cast", "call", contract_address,
    "verifyNamespace(uint64,uint8[],uint8[],uint8[],uint8[],uint8[])",
    str(namespace),
    str(proof_bytes),
    str(commit_bytes),
    str(ns_table_bytes),
    str(tx_comm_bytes),
    str(common_data_bytes),
    "--private-key", private_key,
    "--rpc-url", "http://localhost:8547",
]

subprocess.run(call_namespace_proof_command)