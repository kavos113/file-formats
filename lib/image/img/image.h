#ifndef LIB_IMAGE_IMG_IMAGE_H
#define LIB_IMAGE_IMG_IMAGE_H

#include <vector>
#include <cstdint>

struct pixel
{
    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;
};

class image
{
public:
    uint32_t width;
    uint32_t height;

    std::vector<pixel> data;
};


#endif //LIB_IMAGE_IMG_IMAGE_H