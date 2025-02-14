use std::time::Instant;
use std::fs::File;
use std::io::Write;

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_n = (n as f64).sqrt() as usize;
    
    for i in 2..=sqrt_n {
        if is_prime[i] {
            (i * i..=n).step_by(i).for_each(|j| is_prime[j] = false);
        }
    }

    let mut primes = Vec::with_capacity(n / 10); // Optimización: reserva memoria
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
        }
    }

    primes
}

fn main() {
    let n = 100_000_000;
    let start_time = Instant::now();
    let primes = sieve_of_eratosthenes(n);
    let duration = start_time.elapsed();

    // Escribir en archivo en un solo paso
    let mut file = File::create("output.txt").expect("No se pudo crear el archivo");
    let primes_string: String = primes.iter().map(|p| p.to_string() + "\n").collect();
    file.write_all(primes_string.as_bytes()).expect("No se pudo escribir en el archivo");

    println!("{}", duration.as_millis());
}
