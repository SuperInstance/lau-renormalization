# lau-renormalization

The renormalization group explains why wildly different systems behave identically near critical points. Water boiling, magnets losing magnetization, and agents synchronizing — all follow the same mathematical law. The RG tells you which details matter and which don't.

## The math in 60 seconds

The **renormalization group** studies how physical systems change under scale transformations. The **beta function** β(g) = dg/d(ln μ) describes how coupling constants "flow" as you zoom out. Fixed points β(g*)=0 correspond to scale-invariant theories — phase transitions.

Key results this crate implements:

- **Fixed points:** stable (UV-attractive), unstable (IR-attractive), marginal
- **Critical exponents:** ν, α, β, γ, δ, η — with scaling relation verification (Rushbrooke, Widom, Fisher, Josephson)
- **Wilson-Fisher fixed point:** the non-trivial fixed point in d=4-ε dimensions
- **Universality classes:** Ising 2D/3D, XY, Heisenberg, Potts, mean-field — different systems, same exponents
- **Wilsonian RG:** integrate out high-momentum modes, get effective low-energy theory
- **Real-space RG:** block spin transformations, Migdal-Kadanoff approximation

References: Wilson & Kogut, *The Renormalization Group and the ε Expansion* (1974); Goldenfeld, *Lectures on Phase Transitions and the Renormalization Group* (1992)

## Quick start

```rust
use lau_renormalization::{BetaFunction, FixedPoint, CriticalExponents, UniversalityClass};

// φ⁴ theory beta function
let beta = BetaFunction::phi4();

// Find fixed points
let fixed = beta.find_fixed_points();
let gaussian = fixed.iter().find(|fp| fp.g == 0.0).unwrap();
assert_eq!(gaussian.stability, Stability::Unstable);

// Wilson-Fisher fixed point in d=4-ε
let wf = beta.wilson_fisher(1.0); // ε=1 (d=3)

// Critical exponents for Ising 3D
let ising3d = UniversalityClass::ising_3d();
let nu = ising3d.nu;     // 0.630
let gamma = ising3d.gamma; // 1.237

// Verify scaling relations
assert!(ising3d.verify_rushbrooke()); // α + 2β + γ = 2
assert!(ising3d.verify_widom());      // γ = β(δ-1)
```

## Key types

| Type | What it is |
|------|-----------|
| `BetaFunction` | The RG flow equation β(g) with numerical integrators |
| `FixedPoint` | A point where β(g*)=0 with stability classification |
| `CriticalExponents` | ν, α, β, γ, δ, η with scaling relation verification |
| `WilsonianRG` | Momentum shell integration for effective actions |
| `UniversalityClass` | A named class (Ising, XY, Heisenberg, etc.) with known exponents |
| `RealSpaceRG` | Block spin transformations and decimation |

## Contributing

[Open an issue](https://github.com/SuperInstance/lau-renormalization/issues) or PR.
