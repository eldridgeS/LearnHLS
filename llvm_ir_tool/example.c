  #include <stdio.h>

  int add_and_multiply(int a, int b) {
      int x = a; // store
      int y = b; // store
      int sum = x + y; // load, load, add, store
      int product = x * y; // load, load, mul, store
      printf("Sum: %d, Product: %d\n", sum, product); // call
      return sum + product; // load, load, add
  }

  int main() {
      int result = add_and_multiply(10, 20);
      printf("Final Result: %d\n", result);
      return 0;
  }
