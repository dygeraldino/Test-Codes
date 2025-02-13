import java.io.FileWriter;
import java.io.IOException;

public class sc {
    public static void main(String[] args) {
        int N = 100000000; 
        long startTime = System.currentTimeMillis();

        boolean[] isPrime = new boolean[N + 1];
        for (int i = 0; i <= N; i++) {
            isPrime[i] = true;
        }
        isPrime[0] = isPrime[1] = false;

        int sqrtN = (int) Math.sqrt(N);
        for (int i = 2; i <= sqrtN; i++) {
            if (isPrime[i]) {
                for (int j = i * i; j <= N; j += i) {
                    isPrime[j] = false;
                }
            }
        }

        long endTime = System.currentTimeMillis();
        long executionTime = endTime - startTime;

        try (FileWriter writer = new FileWriter("output.txt")) {
            for (int i = 2; i <= N; i++) {
                if (isPrime[i]) {
                    writer.write(i + "\n");
                }
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
     
        System.out.println(executionTime);
    }
}