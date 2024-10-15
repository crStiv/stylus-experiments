import json
import subprocess
from eth_account import Account
from web3 import Web3

with open('test_data.json', 'r') as f:
    data = json.load(f)

namespace = data["namespace"]
proof_bytes = data["ns_proof"]
commit_bytes = data["vid_commit"]
ns_table_bytes = data["ns_table"]
tx_comm_bytes = data["tx_commit"]
common_data_bytes = data["vid_common"]

contract_address = "0x4e5b65fb12d4165e22f5861d97a33ba45c006114"
private_key = "0xdc04c5399f82306ec4b4d654a342f40e2e0620fe39950d967e1e574b32d4dd36"

w3 = Web3(Web3.HTTPProvider("http://localhost:8547"))

calldata_command = [
    "cast", "abi-encode", "verifyNamespace(uint64,uint8[],uint8[],uint8[],uint8[],uint8[])",
    str(namespace),
    str(proof_bytes),
    str(commit_bytes),
    str(ns_table_bytes),
    str(tx_comm_bytes),
    str(common_data_bytes)
]

calldata_result = subprocess.run(calldata_command, capture_output=True, text=True)
calldata = calldata_result.stdout.strip()

nonce = w3.eth.get_transaction_count(Account.from_key(private_key).address)
transaction = {
    'to': w3.to_checksum_address(contract_address),
    'value': 0,
    'gas': 2000000,
    'gasPrice': w3.to_wei('50', 'gwei'),
    'nonce': nonce,
    'data': calldata
}

signed_txn = w3.eth.account.sign_transaction(transaction, private_key)
txn_hash = w3.eth.send_raw_transaction(signed_txn.raw_transaction)
print("send hash:", txn_hash.hex())

receipt = w3.eth.wait_for_transaction_receipt(txn_hash)
print(receipt)
