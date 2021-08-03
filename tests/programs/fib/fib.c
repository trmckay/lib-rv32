unsigned int fib(unsigned int n) {
    return n < 2 ? 1 : fib(n - 2) + fib(n - 1);
}
