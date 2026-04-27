#include "ppm_parser.h"

#include <iostream>
#include <fstream>

void write_ppm(const image& img, const char* filename)
{
    std::ofstream file(filename, std::ios::out | std::ios::trunc);
    if (!file.is_open())
    {
        std::cerr << "Error: Could not open file for writing: " << filename << std::endl;
        return;
    }

    file << "P3\n" << img.width << " " << img.height << "\n255\n";
    for (uint32_t y = 0; y < img.height; ++y)
    {
        for (uint32_t x = 0; x < img.width; ++x)
        {
            const pixel& p = img.data[y * img.width + x];
            file << static_cast<int>(p.r) << " "
                 << static_cast<int>(p.g) << " "
                 << static_cast<int>(p.b) << " ";
        }
        file << "\n";
    }
}

image read_ppm(const char* filename)
{
    std::ifstream file(filename, std::ios::in);
    if (!file.is_open())
    {
        std::cerr << "Error: Could not open file for reading: " << filename << std::endl;
        return {};
    }

    std::string magic_number;
    file >> magic_number;
    if (magic_number != "P3")
    {
        std::cerr << "Error: Unsupported PPM format (only P3 is supported): " << magic_number << std::endl;
        return {};
    }

    uint32_t width, height, max_color;
    file >> width >> height >> max_color;
    if (max_color != 255)
    {
        std::cerr << "Error: Unsupported max color value (only 255 is supported): " << max_color << std::endl;
        return {};
    }

    image img;
    img.width = width;
    img.height = height;
    img.data.resize(width * height);

    for (uint32_t y = 0; y < height; ++y)
    {
        for (uint32_t x = 0; x < width; ++x)
        {
            int r, g, b;
            file >> r >> g >> b;
            img.data[y * width + x] = { static_cast<uint8_t>(r), static_cast<uint8_t>(g), static_cast<uint8_t>(b), 255 };
        }
    }

    return img;
}
