#ifndef MATRIX_MULT_H
#define MATRIX_MULT_H
#include "ap_fixed.h" // library for fixed point data types


//Define Matrix dimensions [X*Y] [Y*Z]
#define X 4
#define Y 4
#define Z 4

typedef ap_fixed<32, 16> fixed_digit; //define our numbers to have 32 total bits, 16 for integers

// Matrix multiplication function declaration
void matrix_mult(
    fixed_digit A[X][Y],
    fixed_digit B[Y][Z],
    fixed_digit C[X][Z]
);

#endif // MATRIX_MULT_H
