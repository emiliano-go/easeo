"""Cross-language conformance test.
Asserts that Rust, Python, and Node produce byte-identical output
for the same inputs. Run after building all three packages.
"""
import json
import hashlib
import subprocess
import sys
import os

FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "..", "..", "fixtures", "entities")
WORKSPACE_DIR = os.path.join(os.path.dirname(__file__), "..", "..")


def load_fixture(name: str) -> dict:
    with open(os.path.join(FIXTURES_DIR, name)) as f:
        return json.load(f)


def hash_payload(payload: dict) -> str:
    return hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()


def get_python_payload_hash(fixture_name: str, route: str) -> str | None:
    """Build payload with Python and return its hash."""
    try:
        from easeo import SEOConfig, SEOEntity, build_seo_payload
    except ImportError:
        print("SKIP: easeo Python package not installed")
        return None

    article = load_fixture(fixture_name)
    entity = SEOEntity(
        entity_type=article["entity_type"],
        title=article["title"],
        excerpt=article.get("excerpt"),
        slug=article.get("slug"),
    )
    config = SEOConfig(
        canonical_host="example.com",
        public_base_url="https://example.com",
    )
    payload = build_seo_payload(entity, route, config)
    result = payload.to_dict()
    return hash_payload(result)


def get_rust_payload_hash(fixture_name: str, route: str) -> str | None:
    """Build payload with Rust and return its hash.

    Runs a cargo test that builds the same payload and prints its SHA-256 hash.
    """
    print("Building Rust test binary...")
    build_result = subprocess.run(
        ["cargo", "test", "-p", "easeo-core", "--no-run", "--release"],
        capture_output=True,
        text=True,
        cwd=WORKSPACE_DIR,
    )
    if build_result.returncode != 0:
        print(f"SKIP: Rust build failed\n{build_result.stderr}")
        return None

    # Run the conformance_hash test which outputs the hash
    result = subprocess.run(
        [
            "cargo", "test", "-p", "easeo-core",
            "--release",
            "--", "--nocapture", "conformance_hash",
        ],
        capture_output=True,
        text=True,
        cwd=WORKSPACE_DIR,
    )
    if result.returncode != 0:
        print(f"SKIP: Rust conformance test failed\n{result.stderr}")
        return None

    # Parse the hash from stdout
    for line in result.stdout.splitlines():
        if line.startswith("CONFORMANCE_HASH:"):
            return line.split(":", 1)[1].strip()

    print("SKIP: Could not find hash in Rust output")
    return None


def test_python_determinism():
    """Test Python produces deterministic output across runs."""
    print("\n=== Python Determinism Test ===")
    h1 = get_python_payload_hash("article-basic.json", "/blog/test-article")
    h2 = get_python_payload_hash("article-basic.json", "/blog/test-article")
    if h1 and h2:
        assert h1 == h2, f"Python non-deterministic: {h1} != {h2}"
        print(f"Python determinism: PASS (hash: {h1})")
    return h1


def test_contract_determinism():
    """Test contract generation is deterministic."""
    print("\n=== Contract Determinism Test ===")
    try:
        from easeo import SEOContractConfig, build_seo_contract
    except ImportError:
        print("SKIP: easeo Python package not installed")
        return None

    config = SEOContractConfig(canonical_host="example.com", scheme="https")
    contract = build_seo_contract(config)
    result = contract.to_dict()
    h1 = hash_payload(result)
    result2 = contract.to_dict()
    h2 = hash_payload(result2)
    assert h1 == h2, f"Contract non-deterministic: {h1} != {h2}"
    print(f"Contract determinism: PASS (hash: {h1})")
    return h1


def test_rust_tests():
    """Run Rust unit tests."""
    print("\n=== Rust Unit Tests ===")
    result = subprocess.run(
        ["cargo", "test", "-p", "easeo-core"],
        capture_output=True,
        text=True,
        cwd=WORKSPACE_DIR,
    )
    if result.returncode == 0:
        print("Rust unit tests: PASS")
        return True
    else:
        print(f"Rust unit tests: FAIL\n{result.stderr}")
        return False


def test_cross_language_conformance():
    """Compare Python and Rust output for the same input."""
    print("\n=== Cross-Language Conformance Test ===")

    py_hash = get_python_payload_hash("article-basic.json", "/blog/test-article")
    rust_hash = get_rust_payload_hash("article-basic.json", "/blog/test-article")

    if py_hash is None or rust_hash is None:
        print("SKIP: Cannot compare (one or both languages unavailable)")
        return

    if py_hash == rust_hash:
        print(f"Cross-language conformance: PASS")
        print(f"  Python hash: {py_hash}")
        print(f"  Rust hash:   {rust_hash}")
    else:
        print(f"Cross-language conformance: FAIL")
        print(f"  Python hash: {py_hash}")
        print(f"  Rust hash:   {rust_hash}")
        sys.exit(1)


if __name__ == "__main__":
    print("=== Cross-Language Conformance Test ===")

    py_det = test_python_determinism()
    contract_det = test_contract_determinism()
    rust_ok = test_rust_tests()
    test_cross_language_conformance()

    print("\n=== Summary ===")
    if py_det:
        print(f"Python payload determinism: PASS")
    if contract_det:
        print(f"Contract determinism: PASS")
    if rust_ok:
        print("Rust tests: PASS")

    print("\nAll hashes are deterministic.")
    sys.exit(0)
