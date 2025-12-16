pub fn median_mut(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mid = v.len() / 2;
    v.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());
    if v.len() % 2 == 1 {
        v[mid]
    } else {
        let (a, b) = minmax(&v[mid - 1], &v[mid]);
        (a + b) / 2.0
    }
}

pub fn percentile_mut(v: &mut [f64], p: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let p = p.clamp(0.0, 100.0);
    let idx = ((p / 100.0) * (v.len() - 1) as f64).round() as usize;
    v.select_nth_unstable_by(idx, |a, b| a.partial_cmp(b).unwrap());
    v[idx]
}

#[inline]
fn minmax(a: &f64, b: &f64) -> (f64, f64) {
    if a <= b { (*a, *b) } else { (*b, *a) }
}
