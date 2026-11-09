# Independent low-wavenumber D3Q27 directional-symmetry test

**November 9, 2026 — POST-EXPLORATION exact-rational certificate, not a prediction frozen before measurement.** This test concerns a **linear weighted-streaming scalar Fourier symbol**, not the repository's nonlinear collision/update model, not measured acoustic dispersion, and not the physical universe.

For the 27 velocities \(V=\{-1,0,1\}^3\), assign weight \(w_s\) to velocity shell \(s=|\{i:v_i\ne0\}|\) and set \(W=\sum_v w(v)\). Define

\[
K_h(k)=\frac1W\sum_{v\in V}w(v)\cos(h\,k\cdot v),\quad
\Delta_h=K_h(3,4,0)-K_h(5,0,0).
\]

These have **equal directional wavelength** because \(3^2+4^2=5^2\). The rational \(h=1/9,1/27,1/81\) are dimensionless phase resolutions, not a claim of real torus wavevectors; periodic physical modes would require the \(2\pi/L\) factor.

## Exact theorem and witness

| Stencil | W | Directional expansion | Rigorous remainder for 0<h<=1/9 |
|---|---:|---|---|
| Uniform shell weights (1,1,1,1), corresponding to the 27 raw channels | 27 | \(\Delta_h=8h^4-\frac{70}3h^6+R_8\) | \(|R_8|\le\frac{1168129}{30240}h^8\) |
| Artificial tuned control weights (0,12,2,1) | 104 | \(\Delta_h=-\frac{60}{13}h^6+R_8\) | \(|R_8|\le\frac{139949}{8736}h^8\) |

The tuned control satisfies \(w_1=2w_2+8w_3\), canceling fourth-rank anisotropy. It is **not** a valid uniform ternary-occupancy equilibrium under the repo's \(n_s\in\{0,1,2\}\) and \(\rho=E=27\) constraint. These are *symbol* convergence results; no claim is made that retuning the collision model improves its measured acoustics.

Proof: exact enumeration gives equal quadratic directional moments; the fourth-moment difference is \(5184/(27\cdot4!)=8\) for uniform weights and zero for the tuned case; the sixth-moment differences give \(-70/3\) and \(-60/13\). For \(|hk\cdot v|\le7/9\), alternating cosine polynomials \(T_{14}\le\cos\le T_{16}\) give certified rational interval endpoints; \(|R_8|\le |hk\cdot v|^8/8!\) summed over both directions gives the bound.

## Outward integer intervals for 10^20 times the directional difference

| Stencil | h | Lower | Upper |
|---|---|---:|---:|
| Uniform | 1/9 | 117601337830287677 | 117601337830306991 |
| Uniform | 1/27 | 1499327493336217 | 1499327493336218 |
| Uniform | 1/81 | 18576198240021 | 18576198240022 |
| Tuned control | 1/9 | -852164040857798 | -852164040851018 |
| Tuned control | 1/27 | -1188808061117 | -1188808061116 |
| Tuned control | 1/81 | -1633790202 | -1633790201 |

Ratios \(|\Delta_{1/9}|/|\Delta_{1/27}|\) and \(|\Delta_{1/27}|/|\Delta_{1/81}|\) satisfy, respectively, **(78,79)** and **(80,81)** for uniform (limit 81), and **(716,718)** and **(727,729)** for tuned (limit 729).

The Python checker is integer-backed `fractions.Fraction`; it recreates all 27 raw velocities, the moments, Taylor enclosure endpoints, and fault controls before producing `SCALING_20261109.hbp`. The Rust 1.81 checker independently recomputes the directional moments, and CI must pass on its committed head. The local Python check passed; until hosted CI passes, **do not label the Rust replay PASS**. Original repo code, historical receipts, Yang–Mills branches and QNN work are untouched.

## Next scientific gate — actual sliced dynamics

Freeze independent axis/oblique observable predictions *before* invoking the nonlinear stream-plus-collision update, with equal physical wavelength and time, states, gate schedule, falsifiers, and error norm pinned. Then compare measured scaling at \(h=1/9,1/27,1/81\). A separately derived reduction must justify replacing that nonlinear dynamics with this scalar symbol. The full causal rule, finite reconstruction, and physically discriminating observations remain **OPEN**. This result shows a finite model *can approximate* a continuous rotationally invariant observable at long wavelength; it does not establish that the universe is a matrix simulation.
