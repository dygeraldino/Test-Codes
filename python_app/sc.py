import time

def sieve_of_eratosthenes(n):
    is_prime = [True] * (n + 1)
    is_prime[0:2] = [False, False]

    for i in range(2, int(n**0.5) + 1):
        if is_prime[i]:
            is_prime[i*i:n+1:i] = [False] * len(range(i*i, n+1, i))
    primes = [i for i, prime in enumerate(is_prime) if prime]
    return primes

def main():
    N = 100000000  
    start_time = time.time()		
    primes = sieve_of_eratosthenes(N)
    end_time = time.time()
    execution_time = int((end_time - start_time) * 1000)  # Tiempo en milisegundos
    with open('output.txt', 'w') as f:
        for prime in primes:
            f.write(f"{prime}\n")

    print(execution_time)

if __name__ == "__main__":
    main()