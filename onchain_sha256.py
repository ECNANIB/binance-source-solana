#!/usr/bin/env python3
"""Print the sha256 of a deployed Solana program as build.sh compares it.

The program's ProgramData account holds the ELF padded with zeros to the reserved size; the ELF itself
ends at e_shoff + e_shnum * 64. This fetches the account over RPC, cuts the ELF there, checks that the
cut-off tail is all zero, and prints the deploy slot, upgrade authority and sha256.

    python3 onchain_sha256.py <program-id> [--rpc URL]
"""
import argparse, base64, hashlib, json, urllib.request

B58 = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"


def b58(bs: bytes) -> str:
    n, s = int.from_bytes(bs, "big"), ""
    while n:
        n, r = divmod(n, 58)
        s = B58[r] + s
    return "1" * (len(bs) - len(bs.lstrip(b"\0"))) + s


def rpc(url, method, params):
    req = urllib.request.Request(url, json.dumps({"jsonrpc": "2.0", "id": 1, "method": method, "params": params}).encode(),
                                 {"Content-Type": "application/json"})
    r = json.load(urllib.request.urlopen(req, timeout=60))
    if "error" in r:
        raise SystemExit(r["error"])
    return r["result"]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("program_id")
    ap.add_argument("--rpc", default="https://api.mainnet-beta.solana.com")
    a = ap.parse_args()
    prog = rpc(a.rpc, "getAccountInfo", [a.program_id, {"encoding": "jsonParsed"}])["value"]
    pd = prog["data"]["parsed"]["info"]["programData"]
    raw = base64.b64decode(rpc(a.rpc, "getAccountInfo", [pd, {"encoding": "base64"}])["value"]["data"][0])
    slot = int.from_bytes(raw[4:12], "little")
    auth = b58(raw[13:45]) if raw[12] else "none (immutable)"
    elf = raw[45:]
    end = int.from_bytes(elf[40:48], "little") + int.from_bytes(elf[60:62], "little") * 64
    assert not any(elf[end:]), "bytes past the ELF are not zero"
    print(f"program      {a.program_id}")
    print(f"deploy slot  {slot}")
    print(f"authority    {auth}")
    print(f"sha256       {hashlib.sha256(elf[:end]).hexdigest()}")


if __name__ == "__main__":
    main()
