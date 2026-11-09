// Second-seat exact D3Q27 velocity-moment falsifier.
// Rust 1.81, integer-only, no unsafe, no external crates.
// This does not test the model's full interacting dispersion relation.

fn shell_weight(x: i128, y: i128, z: i128, w: [i128; 4]) -> i128 {
    w[(x != 0) as usize + (y != 0) as usize + (z != 0) as usize]
}

fn moment(p: [u32; 3], w: [i128; 4]) -> i128 {
    let mut s = 0_i128;
    for x in -1_i128..=1 {
        for y in -1_i128..=1 {
            for z in -1_i128..=1 {
                s += shell_weight(x, y, z, w)
                    * x.pow(p[0]) * y.pow(p[1]) * z.pow(p[2]);
            }
        }
    }
    s
}

fn directional_sixth(d: [i128; 3], w: [i128; 4]) -> i128 {
    let mut s = 0_i128;
    for x in -1_i128..=1 {
        for y in -1_i128..=1 {
            for z in -1_i128..=1 {
                s += shell_weight(x, y, z, w)
                    * (d[0] * x + d[1] * y + d[2] * z).pow(6);
            }
        }
    }
    s
}

fn certify() -> [i128; 8] {
    // w1 = 2*w2 + 8*w3; this is the fourth-rank isotropy condition.
    let w = [0, 12, 2, 1];
    let m2 = moment([2, 0, 0], w);
    let m4 = moment([4, 0, 0], w);
    let m22 = moment([2, 2, 0], w);
    let m6 = moment([6, 0, 0], w);
    let m42 = moment([4, 2, 0], w);
    let m222 = moment([2, 2, 2], w);
    let d2 = directional_sixth([1, 1, 0], w);
    let d3 = directional_sixth([1, 1, 1], w);
    assert_eq!([m2, m4, m22, m6, m42, m222], [48, 48, 16, 48, 16, 8]);
    assert_eq!(m4, 3 * m22);
    assert_ne!(m6, 5 * m42);
    assert_ne!(m6, 15 * m222);
    assert_eq!([d2, d3], [576, 2304]);
    assert_ne!(d2, 8 * m6);
    assert_ne!(d3, 27 * m6);
    // False-control: an incorrect shell weight breaks rank-four isotropy.
    let wrong = [0, 13, 2, 1];
    assert_eq!(moment([4, 0, 0], wrong), 50);
    assert_ne!(moment([4, 0, 0], wrong), 3 * moment([2, 2, 0], wrong));
    for weights in [[0, 12, 2, 1], [0, 1, 0, 0], [0, 7, 3, 5], [4, 1, 9, 2]] {
        // Exact pointwise identities on {-1,0,1} imply the general no-go.
        assert_eq!(moment([6, 0, 0], weights), moment([4, 0, 0], weights));
        assert_eq!(moment([4, 2, 0], weights), moment([2, 2, 0], weights));
    }
    [m2, m4, m22, m6, m42, m222, d2, d3]
}

fn main() {
    let [m2, m4, m22, m6, m42, m222, d2, d3] = certify();
    println!("D3Q27_RANK4|M2={m2}|M4={m4}|M22={m22}|M4_eq_3M22=1|json=0");
    println!("D3Q27_RANK6|M6={m6}|M42={m42}|M222={m222}|M6_eq_5M42=0|M6_eq_15M222=0|json=0");
    println!(
        "D3Q27_DIRECTIONS|xy_sixth={d2}|xyz_sixth={d3}|axis_times_8={}|axis_times_27={}|json=0",
        8 * m6, 27 * m6
    );
    println!("D3Q27_NO_GO|exact_rank4_and_rank6_nonzero_variance=IMPOSSIBLE|scope=raw_ternary_velocity_moments|json=0");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_exact_moment_checks() {
        certify();
    }
    #[test]
    fn false_fourth_order_control() {
        assert_eq!(moment([4, 0, 0], [0, 13, 2, 1]), 50);
        assert_ne!(50, 3 * 16);
    }
    #[test]
    fn nonzero_variance_forces_sixth_obstruction() {
        let w = [0, 12, 2, 1];
        assert!(moment([2, 0, 0], w) > 0);
        assert_eq!(moment([6, 0, 0], w), moment([4, 0, 0], w));
        assert_eq!(moment([4, 2, 0], w), moment([2, 2, 0], w));
        assert_ne!(moment([6, 0, 0], w), 5 * moment([4, 2, 0], w));
    }
}
