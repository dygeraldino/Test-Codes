const fs = require("fs");

function sieveOfEratosthenes(n) {
  const isPrime = new Uint8Array(n + 1);
  isPrime.fill(1);
  isPrime[0] = isPrime[1] = 0;
  const sqrtN = Math.floor(Math.sqrt(n));

  for (let i = 2; i <= sqrtN; i++) {
    if (isPrime[i]) {
      for (let j = i * i; j <= n; j += i) {
        isPrime[j] = 0;
      }
    }
  }
  const primes = [];
  for (let i = 2; i <= n; i++) {
    if (isPrime[i]) primes.push(i);
  }
  return primes;
}

function main() {
  const N = 100000000;
  const startTime = Date.now();
  const primes = sieveOfEratosthenes(N);
  const endTime = Date.now();
  const executionTime = endTime - startTime;
  fs.writeFileSync("output.txt", primes.join("\n"));

  console.log(executionTime);
}

main();
