

pub fn count_zeros_up_to(t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let t_over_2pi = t / (2.0 * std::f64::consts::PI);
    t_over_2pi * (t_over_2pi / std::f64::consts::E).ln() + 7.0 / 8.0
}

pub fn estimate_zero_height(n: f64) -> f64 {
    let mut lo = 1.0;
    let mut hi = 1e15;
    
    for _ in 0..1000 {
        let mid = lo + (hi - lo) / 2.0;
        let count = count_zeros_up_to(mid);
        
        if (hi - lo) < 0.0001 {
            return mid;
        }
        
        if count < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo + (hi - lo) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_zeros() {
        let n1 = count_zeros_up_to(14.134725);
        assert!((n1 - 0.45).abs() < 0.5, "N(14.134) = {}", n1);
        
        let n2 = count_zeros_up_to(21.022040);
        assert!((n2 - 1.57).abs() < 0.5, "N(21.022) = {}", n2);
        
        let n100 = count_zeros_up_to(100.0);
        assert!((n100 - 29.0).abs() < 1.0, "N(100) = {}", n100);
    }

    #[test]
    fn test_estimate_zeros() {
        let e1 = estimate_zero_height(1.0);
        assert!((e1 - 17.8).abs() < 2.0, "est(1) = {}", e1); 
        
        let e10 = estimate_zero_height(10.0);
        assert!((e10 - 49.77).abs() < 2.0, "est(10) = {}", e10);
    }
}
