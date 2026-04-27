#include "../img/image.h"
#include "../img/format/ppm_parser.h"

int main()
{
    image img;
    img.width = 800;
    img.height = 600;
    img.data.resize(img.width * img.height);

    for (uint32_t y = 0; y < img.height; ++y)
    {
        for (uint32_t x = 0; x < img.width; ++x)
        {
            pixel& p = img.data[y * img.width + x];
            p.r = static_cast<uint8_t>((x * 255) / img.width);
            p.g = static_cast<uint8_t>((y * 255) / img.height);
            p.b = 128;
            p.a = 255;
        }
    }

    write_ppm(img, "gradient.ppm");

    return 0;
}