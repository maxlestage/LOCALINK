#!/usr/bin/env python3
"""App Store Connect, from GitHub Actions, with an API key and nothing else.

    asc.py bundle <identifier> <name>   creates the bundle ID if it does not exist and
                                        prints the Team ID (its "seedId")
    asc.py app <identifier>             prints the id of the app record, or nothing

The key comes from the environment: ASC_KEY_ID, ASC_ISSUER_ID and ASC_KEY_P8 (the
content of the .p8 file, even pasted on one line). Only the standard library and the
`openssl` command are used: the JSON Web Token is signed with ES256.
"""

import base64
import json
import os
import re
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request

API = "https://api.appstoreconnect.apple.com/v1"


def b64url(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()


def pem(raw: str) -> str:
    """The key as a proper PEM, whatever happened to its line breaks."""
    body = re.sub(r"-----[A-Z ]+-----", "", raw)
    body = re.sub(r"\s+", "", body.replace("\\n", ""))
    lines = [body[i : i + 64] for i in range(0, len(body), 64)]
    return "-----BEGIN PRIVATE KEY-----\n" + "\n".join(lines) + "\n-----END PRIVATE KEY-----\n"


def der_to_raw(der: bytes) -> bytes:
    """An ECDSA signature in DER (what openssl writes) as r||s (what JWT wants)."""
    if der[0] != 0x30:
        raise ValueError("not a DER sequence")
    i = 2 if der[1] < 0x80 else 2 + (der[1] & 0x7F)
    parts = []
    for _ in range(2):
        if der[i] != 0x02:
            raise ValueError("not a DER integer")
        length = der[i + 1]
        value = der[i + 2 : i + 2 + length].lstrip(b"\x00")
        parts.append(value.rjust(32, b"\x00"))
        i += 2 + length
    return parts[0] + parts[1]


def token(key_id: str, issuer: str, key_pem: str) -> str:
    header = b64url(json.dumps({"alg": "ES256", "kid": key_id, "typ": "JWT"}).encode())
    now = int(time.time())
    claims = {"iss": issuer, "iat": now, "exp": now + 15 * 60, "aud": "appstoreconnect-v1"}
    payload = b64url(json.dumps(claims).encode())
    signing_input = f"{header}.{payload}".encode()
    with tempfile.NamedTemporaryFile("w", suffix=".p8", delete=False) as f:
        f.write(key_pem)
        path = f.name
    try:
        der = subprocess.run(
            ["openssl", "dgst", "-sha256", "-sign", path],
            input=signing_input,
            capture_output=True,
            check=True,
        ).stdout
    finally:
        os.unlink(path)
    return f"{header}.{payload}.{b64url(der_to_raw(der))}"


def call(method: str, path: str, body=None):
    jwt = token(os.environ["ASC_KEY_ID"].strip(), os.environ["ASC_ISSUER_ID"].strip(), pem(os.environ["ASC_KEY_P8"]))
    data = json.dumps(body).encode() if body is not None else None
    request = urllib.request.Request(API + path, data=data, method=method)
    request.add_header("Authorization", f"Bearer {jwt}")
    request.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        details = error.read().decode(errors="replace")
        sys.exit(f"App Store Connect answered {error.code} to {method} {path}: {details}")


def find_bundle(identifier: str):
    query = urllib.parse.urlencode({"filter[identifier]": identifier, "limit": 200})
    for item in call("GET", f"/bundleIds?{query}")["data"]:
        if item["attributes"]["identifier"] == identifier:
            return item
    return None


def bundle(identifier: str, name: str) -> None:
    item = find_bundle(identifier)
    if item is None:
        body = {
            "data": {
                "type": "bundleIds",
                "attributes": {"identifier": identifier, "name": name, "platform": "IOS"},
            }
        }
        item = call("POST", "/bundleIds", body)["data"]
        print(f"bundle ID {identifier} created", file=sys.stderr)
    else:
        print(f"bundle ID {identifier} found", file=sys.stderr)
    print(item["attributes"]["seedId"])


def app(identifier: str) -> None:
    query = urllib.parse.urlencode({"filter[bundleId]": identifier})
    for item in call("GET", f"/apps?{query}")["data"]:
        if item["attributes"]["bundleId"] == identifier:
            print(item["id"])
            return


if __name__ == "__main__":
    if len(sys.argv) >= 4 and sys.argv[1] == "bundle":
        bundle(sys.argv[2], sys.argv[3])
    elif len(sys.argv) >= 3 and sys.argv[1] == "app":
        app(sys.argv[2])
    else:
        sys.exit(__doc__)
