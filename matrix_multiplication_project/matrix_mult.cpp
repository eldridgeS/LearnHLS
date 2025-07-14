#include "matrix_mult.h" // Include the header for your HLS function



// Top function definition
void matrix_mult(fixed_digit A[X][Y], fixed_digit B[Y][Z], fixed_digit C[X][Z]){

    #pragma HLS DATAFLOW // Apply DATAFLOW to the top-level function, good for nested loops>>>>>>>>>>>>>>>>>>>>>>>

//Breaks the arrays into registers instead of BRAMs so every digit can be accessed in single clock cycle>>>>>>>>>>>
	#pragma HLS ARRAY_PARTITION variable=A complete dim=1
	#pragma HLS ARRAY_PARTITION variable=A complete dim=2
	#pragma HLS ARRAY_PARTITION variable=B complete dim=1
	#pragma HLS ARRAY_PARTITION variable=B complete dim=2
	#pragma HLS ARRAY_PARTITION variable=C complete dim=1
	#pragma HLS ARRAY_PARTITION variable=C complete dim=2

	row_loop: for (int i = 0; i < X; i++) {

		col_loop: for (int j = 0; j < Z; j++) {
			#pragma HLS pipeline II=1 //each matrix position calculation starts every cycle>>>>>>>>>>>>>>>>>>>>>>>>>

			fixed_digit sum = 0;

			product_loop: for (int k = 0; k < Y; k++) {
				#pragma HLS UNROLL //unroll inner loop to allow parallelism>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
				sum += A[i][k] * B[k][j];
			}
			C[i][j] = sum;
 		}
	}
}
