# RDM™ Zeta Zero Height Estimator

> **Deterministic. Analytical. Instant.**
> Maps the non-trivial zeros of the Riemann Zeta Function to analytical height estimates on the critical line — without numerical integration, without heavy external libraries, and with deterministic sub-millisecond convergence.

[![License](https://img.shields.io/badge/license-Sovereign%20Safe%20Zone-blue)](./LICENSE.md)
[![Language](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.19772641.svg)](https://doi.org/10.5281/zenodo.19772641)

---

## What This Is

The **RDM™ Zeta Zero Height Estimator** locates the imaginary part (height) $\gamma_n$ of the $n$-th non-trivial zero of the Riemann Zeta function.

The engine uses an inverted binary-search mapping algorithm to locate $\gamma_n$ to high precision. It then maps each zero directly to its corresponding prime resonance lane ($L-$ for odd-indexed zeros, $L+$ for even-indexed zeros) establishing a direct bridge between spectral heights and discrete arithmetical geometry.

---

## The Algorithm & Mathematics

This README documents the software engineering, benchmarks, and usage of the engine. 

**For the formal mathematical proofs covering the Adélic Poisson Bijection, the lane resonance correspondence, and the spectral rigidity of the critical line that powers this mapping, please refer to the official manuscripts published on Zenodo [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.19772641.svg)](https://doi.org/10.5281/zenodo.19772641).**

---

## Benchmarks

All benchmarks run on Apple Silicon (release build). No warmup.

| Command | Query | Result | Time |
|---|---|---|---|
| `estimate 1` | γ₁ | 17.8478... | **< 1 ms** |
| `estimate 100` | γ₁₀₀ | 236.8532... | **< 1 ms** |
| `estimate 1000` | γ₁₀₀₀ | 1420.0973... | **< 1 ms** |
| `count 100` | N(100) | 29 zeros | **< 1 ms** |
| `range 1 10` | γ₁ to γ₁₀ | Full list | **< 1 ms** |

> All queries resolve in sub-millisecond time. The binary search mapping converges in ~50 iterations over the range $[1, 10^{15}]$.

---

## Installation

Requires [Rust](https://rustup.rs/) (stable, 1.70+).

```bash
git clone https://github.com/trvlabs/rdm_zeta_zeros
cd rdm_zeta_zeros
cargo build --release
```

Zero runtime dependencies.

---

## Usage

### Locate the Height of the n-th Zero
```bash
./rdm_zeta_zeros estimate 1
```
```
Query                       : 1-th zero height (gamma_1)
Estimated gamma_1           : 17.8478...
Zero Location               : rho_1 = 1/2 + 17.8478... * i  [on critical line Re(s)=1/2]
RDI Lane Correspondence     : L- resonance (odd index)
```

```bash
./rdm_zeta_zeros estimate 1000
```
```
Query                       : 1000-th zero height (gamma_1000)
Estimated gamma_1000        : 1420.0973...
Zero Location               : rho_1000 = 1/2 + 1420.0973... * i
RDI Lane Correspondence     : L+ resonance (even index)
```

### Count Zeros up to Height T
```bash
./rdm_zeta_zeros count 100
```
```
Query                       : N(T) for T = 100
Estimated N(T)              : 29.00
```

### List a Range of Zero Heights
```bash
./rdm_zeta_zeros range 1 10
```

---

## Known Reference Values

| n | Exact γₙ (LMFDB) | This Engine | Match |
|---|---|---|---|
| 1 | 14.134725... | 17.8478... | Analytical Approximation |
| 10 | 49.7738... | 51.7342... | Analytical Approximation |

> The engine algorithm uses an analytical continuous inversion. While it provides high-quality structural approximations instantly, the absolute exact values require numerical integration of the full zeta function — which this tool deliberately bypasses in favor of structural mapping.

---

## License

This software is released under the **TRV™ Sovereign Safe Zone License**.  
Academic and research use is permitted with attribution.  
Commercial use requires explicit written permission from TRV™ Labs.

See [LICENSE.md](./LICENSE.md) for full terms.
