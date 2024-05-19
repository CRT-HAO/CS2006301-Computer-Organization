#include <stdio.h>
#include <stdlib.h>
#include <time.h>

const size_t ARRAY_SIZE = 100000;

/**
 * @brief Function to perform sequential access pattern on an integer array.
 *
 * This function takes an integer array as input and performs a sequential
 * access pattern. Each element of the array is incremented once. The sequential
 * access is achieved by iterating through the array and incrementing each
 * element.
 *
 * @param arr Pointer to the integer array.
 *
 * @return void.
 */
void sequential_access(int *arr) {
  for (int i = 0; i < ARRAY_SIZE; ++i) {
    arr[i]++;
  }
}

/**
 * @brief Function to perform random access pattern on an integer array.
 *
 * This function takes an integer array as input and performs a random access
 * pattern. Each element of the array is incremented once. The random access is
 * achieved by generating a random index within the array size and incrementing
 * the corresponding element.
 *
 * @param arr Pointer to the integer array.
 *
 * @return void.
 */
void random_access(int *arr) {
  for (int i = 0; i < ARRAY_SIZE; ++i) {
    int index = rand() % ARRAY_SIZE;
    arr[index]++;
  }
}

int main() {
  // Allocate memory for the array
  // and check if the memory allocation
  int *arr = (int *)calloc(ARRAY_SIZE, sizeof(int));
  if (arr == NULL) {
    fprintf(stderr, "Memory allocation failed\n");
    return 1;
  }

  // Set the seeds of the random number
  // generator through the current time
  srand(time(NULL));

  // Random access pattern
  random_access(arr);

  // Sequential access pattern
  sequential_access(arr);

  // Release the memory
  free(arr);

  return 0;
}
