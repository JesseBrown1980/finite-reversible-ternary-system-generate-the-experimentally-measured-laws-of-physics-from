//! Independent D3Q27 integer velocity-moment audit of low-k anisotropy.
//! This checks a weighted scalar streaming symbol, NOT nonlinear collisions.
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic, clippy::float_cmp)]

fn shell_weight(v: [i128; 3], w: [i128; 4]) -> i128 {
    let n = v.iter().filter(|&&x| x != 0).count();
    w[n]
}

fn moment(k: [i128; 3], order: u32, w: [i128; 4]) -> i128 {
    let mut result = 0;
    for x in -1_i128..=1 {
        for y in -1_i128..=1 {
            for z in -1_i128..=1 {
                let v = [x, y, z];
                let kv = k[0] * x + k[1] * y + k[2] * z;
                result += shell_weight(v, w) * kv.pow(order);
            }
        }
    }
    result
}

fn weight(w: [i128; 4]) -> i128 {
    w[0] + 6 * w[1] + 12 * w[2] + 8 * w[3]
}

fn check(w: [i128; 4], expected_weight: i128, d4: i128, d6: i128) {
    let a = [5, 0, 0];
    let b = [3, 4, 0];
    assert_eq!(a.iter().map(|x| x * x).sum::<i128>(), 25);
    assert_eq!(b.iter().map(|x| x * x).sum::<i128>(), 25);
    assert_eq!(weight(w), expected_weight);
    let delta2 = moment(b, 2, w) - moment(a, 2, w);
    let delta4 = moment(b, 4, w) - moment(a, 4, w);
    let delta6 = moment(b, 6, w) - moment(a, 6, w);
    assert_eq!(delta2, 0);
    assert_eq!(delta4, d4);
    assert_eq!(delta6, d6);
    for k in [a, b, [0, 5, 0], [4, -3, 0]] {
        let swapped = [k[1], k[0], -k[2]];
        assert_eq!(moment(k, 2, w), moment(swapped, 2, w));
        assert_eq!(moment(k, 4, w), moment(swapped, 4, w));
        assert_eq!(moment(k, 6, w), moment(swapped, 6, w));
        assert_eq!(moment(k, 6, w), moment([-k[0], -k[1], -k[2]], 6, w));
    }
}

fn run() {
    let uniform = [1, 1, 1, 1];
    let tuned = [0, 12, 2, 1];
    check(uniform, 27, 5_184, 453_600);
    check(tuned, 104, 0, 345_600);
    assert_eq!(tuned[1], 2 * tuned[2] + 8 * tuned[3]);
    let defective = [0, 13, 2, 1];
    assert_ne!(defective[1], 2 * defective[2] + 8 * defective[3]);
    assert_eq!(5_184_i128, 8 * 24 * 27);
    assert_eq!(453_600_i128 * 3, 70 * 720 * 27);
    assert_eq!(345_600_i128 * 13, 60 * 720 * 104);
}

fn main() {
    run();
    println!("SCALING_RUST|models=2|exact_order=6|independent_moments=PASS|nonlinear_dynamics=UNTESTED|json=0");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_both_models() { run(); }

    #[test]
    fn wrong_shell_is_detected() {
        let w = [0, 13, 2, 1];
        assert_eq!(moment([3, 4, 0], 4, w) - moment([5, 0, 0], 4, w), -576);
        assert_ne!(moment([5, 0, 0], 4, w), moment([5, 0, 0], 4, [0, 12, 2, 1]));
    }
}
