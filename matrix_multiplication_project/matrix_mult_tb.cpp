#include <iostream> // For standard input/output
#include <iomanip>  // For setting output precision
#include "matrix_mult.h" // Include the header for your HLS function



// Main function for the test bench
int main() {

    // Declare matrices for input and output
    fixed_digit A[X][Y];
    fixed_digit B[Y][Z];
    fixed_digit C_hls[X][Z]; // HLS result
    fixed_digit C_ctrl[X][Z]; // Software control result

    // Initialize matrices A and B with some sample data
    for (int i = 0; i < X; i++) {
        for (int j = 0; j < Y; j++) {
            A[i][j] = (fixed_digit)(i + j + 1); //Initialized with random data
        }
    }
    for (int i = 0; i < Y; i++) {
        for (int j = 0; j < Z; j++) {
            B[i][j] = (fixed_digit)(i * 2 - j); //Initialized with random data
        }
    }

    // Calculate the control reference using software
    std::cout << "Calculating software reference result..." << std::endl;
    for (int i = 0; i < X; i++) {
        for (int j = 0; j < Z; j++) {
            fixed_digit sum_ctrl = 0;
            for (int k = 0; k < Y; k++) {
                sum_ctrl += A[i][k] * B[k][j];
            }
            C_ctrl[i][j] = sum_ctrl;
        }
    }

    // Call the HLS function
    std::cout << "Calling HLS matrix_mult function..." << std::endl;
    matrix_mult(A, B, C_hls);

    // Compare the results
    int errors = 0;
    std::cout << "\\nVerifying results..." << std::endl;
    for (int i = 0; i < X; i++) {
        for (int j = 0; j < Z; j++) {
            if (C_hls[i][j] != C_ctrl[i][j]) {
                std::cerr << "ERROR: Mismatch at C[" << i << "][" << j << "]: ";
                std::cerr << "HLS = " << std::fixed << std::setprecision(4) << (double)C_hls[i][j] << ", ";
                std::cerr << "Control = " << std::fixed << std::setprecision(4) << (double)C_ctrl[i][j] << std::endl;
                errors++;
            }
        }
    }


    if (errors == 0) {
        std::cout << "\\nTEST PASSED! All results match." << std::endl;
        return 0; // Indicate success
    } else {
        std::cerr << "\\nTEST FAILED! " << errors << " errors found." << std::endl;
        return 1; // Indicate failure
    }
}
