#ifndef BOX_BLUR_H
#define BOX_BLUR_H

// Define image dimensions and blur kernel size
#define IMAGE_WIDTH  640
#define IMAGE_HEIGHT 480
#define BLUR_SIZE    3 //3x3 blur kernel

// Function prototype for the Box Blur filter
// input_pixels: 2D array representing the input RGB image (Height x Width*3 for R,G,B)
// output_pixels: 2D array representing the output RGB image (Height x Width*3 for R,G,B)
//extern "C" so g++ compiler exports the function with C linkage without name mangling
extern "C" void box_blur_filter(unsigned char input_pixels[IMAGE_HEIGHT][IMAGE_WIDTH * 3],
                     unsigned char output_pixels[IMAGE_HEIGHT][IMAGE_WIDTH * 3]);

#endif // BOX_BLUR_H
