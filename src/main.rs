use std::env;
use rdm_zeta_zeros::api::*;

fn print_usage(prog: &str) {
    println!("Usage: {} <command> [args]", prog);
    println!();
    println!("Commands:");
    println!("  count <T>          Number of zeros with imaginary part <= T");
    println!("  estimate <n>       Estimate imaginary part (height) of the n-th zero");
    println!("  range <n1> <n2>    List estimated heights from index n1 to n2");
    println!();
    println!("Examples:");
    println!("  {} estimate 10", prog);
    println!("  {} count 100", prog);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];

    if args.len() < 3 {
        print_usage(prog);
        return;
    }

    match args[1].as_str() {
        "count" => {
            let t: f64 = match args[2].parse() {
                Ok(v) if v > 0.0 => v,
                _ => { println!("Error: T must be > 0"); return; }
            };
            let n = count_zeros_up_to(t);
            println!("============================================================");
            println!("  RDM(TM) ZETA ZERO HEIGHT ESTIMATOR -- Uchechukwu RH Bijection");
            println!("============================================================");
            println!("Query                       : N(T) for T = {}", t);
            println!("Von Mangoldt Formula        : N(T) = (T/2pi)*ln(T/2pi*e) + 7/8");
            println!("Estimated N(T)              : {:.2}", n);
            println!("Proof Basis                 : Uchechukwu RH Proof -- Adelic Poisson Bijection");
            println!("============================================================");
        }
        "estimate" => {
            let n: f64 = match args[2].parse() {
                Ok(v) if v >= 1.0 => v,
                _ => { println!("Error: n must be >= 1"); return; }
            };
            let n_int = n as u64;
            let est = estimate_zero_height(n);
            let lane = if n_int % 2 == 0 { "L+ resonance (even index)" } else { "L- resonance (odd index)" };
            
            println!("============================================================");
            println!("  RDM(TM) ZETA ZERO HEIGHT ESTIMATOR -- Uchechukwu RH Bijection");
            println!("============================================================");
            println!("Query                       : {}-th zero height (gamma_{})", n_int, n_int);
            println!("Von Mangoldt Formula        : N(T) = (T/2pi)*ln(T/2pi*e) + 7/8");
            println!("Binary Search Range         : [1.0, 1e15]");
            println!("Estimated gamma_{:<13}: {:.4}...", n_int, est);
            println!("Zero Location               : rho_{} = 1/2 + {:.4}... * i  [on critical line Re(s)=1/2]", n_int, est);
            println!("RDI Lane Correspondence     : {}", lane);
            println!("Proof Basis                 : Uchechukwu RH Proof -- Adelic Poisson Bijection");
            println!("============================================================");
        }
        "range" => {
            if args.len() < 4 {
                println!("Error: range requires n1 and n2");
                return;
            }
            let n1: u64 = args[2].parse().unwrap_or(1);
            let n2: u64 = args[3].parse().unwrap_or(1);
            if n1 > n2 || n1 < 1 {
                println!("Error: invalid range");
                return;
            }
            
            println!("============================================================");
            println!("  RDM(TM) ZETA ZERO HEIGHT ESTIMATOR -- Uchechukwu RH Bijection");
            println!("============================================================");
            for n in n1..=n2 {
                let est = estimate_zero_height(n as f64);
                let lane = if n % 2 == 0 { "L+" } else { "L-" };
                println!("n={:<5} | gamma_{:<5} approx {:.4} | Lane: {}", n, n, est, lane);
            }
            println!("============================================================");
        }
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            print_usage(prog);
        }
    }
}
