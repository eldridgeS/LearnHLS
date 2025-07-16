#include "box_blur.h"

// Box Blur filter function
// Takes an RGB image and produces an RGB blurred image
void box_blur_filter(unsigned char input_pixels[IMAGE_HEIGHT][IMAGE_WIDTH * 3],
                     unsigned char output_pixels[IMAGE_HEIGHT][IMAGE_WIDTH * 3]) {


    /** HLS INTERFACE pragmas: These pragmas tell HLS to map the return, input and output
     to an AXI4-Lite control interface, so that a processor can control the function and
     access these arrays by reading and writing to memory-mapped registers **/
    #pragma HLS INTERFACE s_axilite port=return bundle=control
    #pragma HLS INTERFACE s_axilite port=input_pixels bundle=control
    #pragma HLS INTERFACE s_axilite port=output_pixels bundle=control
   	//#pragma HLS INTERFACE mode=m_axi port=input_pixels depth=1000000 //rounded up image_height*width*3
    //#pragma HLS INTERFACE mode=m_axi port=output_pixels depth=1000000


    //Create a local 2D buffer array crucial for 2D convolutions to enable pipelining.
    unsigned char line_buffer[BLUR_SIZE][IMAGE_WIDTH * 3];
    //partition buffefr array allow its rows to allow simultaneous reading
    #pragma HLS ARRAY_PARTITION variable=line_buffer complete dim=1

    // Loop through each row
    for (int row = 0; row < IMAGE_HEIGHT; row++) {

    	//Logic for updating line Buffer
        // Shift existing rows in the line buffer up by one
        for (int i = 0; i < BLUR_SIZE - 1; i++) {
            for (int j = 0; j < IMAGE_WIDTH * 3; j++) {
                line_buffer[i][j] = line_buffer[i + 1][j];
            }
        }
        // Load the new row from the input image into the last row of the line buffer
        for (int j = 0; j < IMAGE_WIDTH * 3; j++) {
            line_buffer[BLUR_SIZE - 1][j] = input_pixels[row][j];
        }


        for (int col = 0; col < IMAGE_WIDTH; col++) {
            // Only start processing once enough rows are in the buffer (i.e., after the first BLUR_SIZE-1 rows)
            if (row >= BLUR_SIZE - 1) {
                int sum_r = 0;
                int sum_g = 0;
                int sum_b = 0;
                int pixel_count = 0;

                // Loop through the blur kernel (e.g., for 3x3 window 3/2 = 1.5 = 1)
                int kernel_offset = BLUR_SIZE / 2;

                //All 9 iterations will be executed in parallel
                for (int k_row = -kernel_offset; k_row <= kernel_offset; k_row++) {
                    #pragma HLS UNROLL // Fully unroll this loop (3 iterations become parallel)
                    for (int k_col = -kernel_offset; k_col <= kernel_offset; k_col++) {
                        #pragma HLS UNROLL // Fully unroll this loop (3 iterations become parallel)

                        // Calculate pixel coordinates
                        int lb_row_idx = k_row + kernel_offset;
                        int lb_col_idx_base = (col + k_col) * 3;
                        // Ensures we don't read outside the valid column range of the line buffer
                        int actual_col_r = (lb_col_idx_base < 0) ? 0 : ((lb_col_idx_base >= IMAGE_WIDTH * 3) ? (IMAGE_WIDTH * 3 - 3) : lb_col_idx_base);
                        int actual_col_g = actual_col_r + 1;
                        int actual_col_b = actual_col_r + 2;


                        // Accumulate RGB values from the neighborhood by reading from the line_buffer
                        sum_r += line_buffer[lb_row_idx][actual_col_r];
                        sum_g += line_buffer[lb_row_idx][actual_col_g];
                        sum_b += line_buffer[lb_row_idx][actual_col_b];
                        pixel_count++;
                    }
                }

                // Calculate average RGB values
                unsigned char avg_r = (unsigned char)(sum_r / pixel_count);
                unsigned char avg_g = (unsigned char)(sum_g / pixel_count);
                unsigned char avg_b = (unsigned char)(sum_b / pixel_count);

                // Store the resulting RGB pixel in the output array
                // Output starts BLUR_SIZE - 1 rows later due to line buffer fill-up
                output_pixels[row - (BLUR_SIZE - 1)][col * 3]     = avg_r;
                output_pixels[row - (BLUR_SIZE - 1)][col * 3 + 1] = avg_g;
                output_pixels[row - (BLUR_SIZE - 1)][col * 3 + 2] = avg_b;
            }
        }
    }
}
