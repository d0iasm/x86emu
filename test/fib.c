int fib(int);

int main() {
  return fib(10);
}

int fib(int n) {
  if (n <= 1)
    return n;
  return fib(n-1) + fib(n-2);
}
