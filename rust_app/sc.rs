use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 2..=sqrt_n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let primes = (2..=n).filter(|&i| is_prime[i]).collect();
    primes
}

fn main() {
    let n = 100_000_000; // Ajusta este valor según sea necesario
    let start_time = Instant::now();
    let primes = sieve_of_eratosthenes(n);
    let total: usize = primes.iter().sum();

    let mut file = File::create("output.txt").expect("No se pudo crear el archivo");
    writeln!(file, "{}", total).expect("No se pudo escribir en el archivo");

    let duration = start_time.elapsed();
    let execution_time = duration.as_millis();

    println!("{}", execution_time);
}
