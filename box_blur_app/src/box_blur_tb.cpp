#include <iostream>
#include <fstream>
#include <vector>
#include "box_blur.h"

// Function to create a dummy RGB image with a simple pattern
void create_image(unsigned char image[IMAGE_HEIGHT][IMAGE_WIDTH * 3]) {
    for (int row = 0; row < IMAGE_HEIGHT; row++) {
        for (int col = 0; col < IMAGE_WIDTH; col++) {
            if (col % 20 < 10) { // Vertical stripes
                image[row][col * 3]     = 255; // Red
                image[row][col * 3 + 1] = 0;   // Green
                image[row][col * 3 + 2] = 0;   // Blue

            } else if (row % 20 < 10) { // Vertical stripes
                image[row][col * 3]     = 255;
                image[row][col * 3 + 1] = 0;
                image[row][col * 3 + 2] = 0;
            } else {
                image[row][col * 3]     = 0;
                image[row][col * 3 + 1] = 255;
                image[row][col * 3 + 2] = 0;
            }
        }
    }
}

// Function to save an RGB image to a simple PPM (Portable PixMap) format
void save_ppm(const char* filename, unsigned char image[IMAGE_HEIGHT][IMAGE_WIDTH * 3]) {
    std::ofstream ofs(filename, std::ios_base::out | std::ios_base::binary);
    if (!ofs.is_open()) {
        std::cerr << "Error: Could not open file " << filename << " for writing." << std::endl;
        return;
    }
    // PPM header: P6 (magic number), width, height, max_pixel_value
    ofs << "P6\n" << IMAGE_WIDTH << " " << IMAGE_HEIGHT << "\n255\n";
    for (int row = 0; row < IMAGE_HEIGHT; ++row) {
        ofs.write((char*)image[row], IMAGE_WIDTH * 3); // Write row by row (R,G,B for each pixel)
    }
    ofs.close();
    std::cout << "Image saved to " << filename << std::endl;
}


int main() {
    // Declare input and output arrays based on defined dimensions
    unsigned char input_image[IMAGE_HEIGHT][IMAGE_WIDTH * 3];
    unsigned char output_image[IMAGE_HEIGHT][IMAGE_WIDTH * 3];

    std::cout << "--- Box Blur Filter Test Bench ---" << std::endl;

    std::cout << "Creating dummy input image..." << std::endl;
    create_image(input_image);
    std::cout << "Dummy input image created." << std::endl;

    // Save the original RGB input image as PPM for image comparison
    std::cout << "Saving original RGB input image to 'input_image.ppm'..." << std::endl;
    save_ppm("input_image.ppm", input_image);

    std::cout << "Calling box_blur_filter (C/C++ simulation)..." << std::endl;
    // Call the box blur filter function
    box_blur_filter(input_image, output_image);
    std::cout << "box_blur_filter call complete." << std::endl;

    // Save the output RGB blurred image as PPM
    std::cout << "Saving output RGB blurred image to 'output_box_blur.ppm'..." << std::endl;
    save_ppm("output_box_blur.ppm", output_image);

    std::cout << "Test bench finished. Please check 'input_image.ppm' and 'output_box_blur.ppm' for visual verification." << std::endl;

    return 0;
}
