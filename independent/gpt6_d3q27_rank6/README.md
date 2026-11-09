# Independent D3Q27 rank-six rotational-isotropy falsifier

**2026-11-09.** Independent second-seat mathematical test, not a claim about the microscopic physical universe. The original model's **exact cubic \(O_h\)** symmetry, fourth-order isotropy condition and dynamical results remain unchanged. This test distinguishes *finite cubic symmetry* from exact *continuous* \(SO(3)\) invariance of **raw sixth-order velocity moments**.

## Exact statement

For \(V=\{-1,0,+1\}^3\), define \(M_{abc}=\sum_{v\in V}w(v)v_x^a v_y^b v_z^c\), with any cubic-symmetric shell weights (the weight depends only on the number of nonzero velocity components). Pointwise, \(v_x^6=v_x^4=v_x^2\) and \(v_x^4v_y^2=v_x^2v_y^2\), so \(M_{600}=M_{400}=M_{200}=A\) and \(M_{420}=M_{220}=B\). Exact fourth-order rotational isotropy requires \(A=3B\), while exact sixth-order rotational isotropy requires \(A=5B\). Both imply \(A=B=0\) and hence \(M_{200}=0\). **Thus a nondegenerate D3Q27 raw velocity measure cannot have both exact fourth-order and sixth-order rotational isotropy.** This is an algebraic obstruction, not a numerical approximation or a dynamical claim.

## Exact integer witness

Use shell weights \((w_0,w_1,w_2,w_3)=(0,12,2,1)\). The original fourth-order condition \(w_1=2w_2+8w_3\) is satisfied. Independent enumeration of all 27 velocities gives:

| Moment | Value |
|---|---:|
| \(M_{200}=M_{400}=M_{600}\) | 48 |
| \(M_{220}=M_{420}\) | 16 |
| \(M_{222}\) | 8 |
| \(\sum_v w(v)(v_x+v_y)^6\) | 576 |
| \(\sum_v w(v)(v_x+v_y+v_z)^6\) | 2304 |

Fourth-rank isotropy holds, \(48=3(16)\). Sixth rank fails, \(48\ne5(16)=80\). The sixth directional sums are inconsistent with isotropy: face diagonal should equal \(8(48)=384\) rather than 576; body diagonal should equal \(27(48)=1296\) rather than 2304. A false-control weight \(w_1=13\) breaks fourth rank as expected.

**Independent evidence:** Python standard-library integer enumeration, completed 2026-11-09, produced the 4-row HBP receipt and SHA-256 sidecar. The Rust 1.81 crate is a separate hosted replay target; the receipt is **post-exploration**, not a prediction-first measurement. CI must compile it, run all tests and clippy, and compare output byte-for-byte before marking the Rust replay PASS. No floating point or JSON calculation occurs.

## Scope and follow-up

This theorem rules out exact rank-six SO(3) symmetry of **raw D3Q27 moments**, not an effective continuum approximation, an interacting model with higher-order cancellations, nonlinear fluid dynamics, or the possibility of a finite sliced physical universe. In particular exact \(O_h\) equivariance does not entail all-order \(SO(3)\) isotropy. A finite universe hypothesis can be tested directly at finite size and finite observer resolution; requiring infinite physical volume is unnecessary here.

Next independent gates: (1) exact local forward/reverse slice rule, with loss of reconstructability returning Held; (2) frozen low-k directional dispersion and observable-error predictions across axes, face/body diagonals, scales 3 and 9; (3) direct \(3^k\) time evolution vs certified jumps and blocking, *without* applying linear jumps to nonlinear rules absent a reduction theorem; (4) when a quantum/gauge claim is made, distinguish Euclidean reflection-positive transfer from real-time unitary motion, then test isolated gauge-invariant spectral weight; (5) preregister dimensionful predictions against experimental data before claims about physical spacetime.
