package main

import (
    "fmt"
    "time"
    "os"
    "strconv"
)

func sieveOfEratosthenes(n int) []int {
    isPrime := make([]bool, n+1)
    for i := 2; i <= n; i++ {
        isPrime[i] = true
    }
    for i := 2; i*i <= n; i++ {
        if isPrime[i] {
            for j := i * i; j <= n; j += i {
                isPrime[j] = false
            }
        }
    }
    primes := []int{}
    for i := 2; i <= n; i++ {
        if isPrime[i] {
            primes = append(primes, i)
        }
    }
    return primes
}

func main() {
    N := 100000000 // Ajusta este valor según sea necesario
    start := time.Now()
    primes := sieveOfEratosthenes(N)
    total := 0
    for _, p := range primes {
        total += p
    }
    f, _ := os.Create("output.txt")
    f.WriteString(strconv.Itoa(total) + "\n")
    f.Close()
    elapsed := time.Since(start)
    fmt.Println(int(elapsed.Milliseconds()))
}