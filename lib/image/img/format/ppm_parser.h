#ifndef LIB_IMAGE_IMG_FORMAT_PPM_PARSER_H
#define LIB_IMAGE_IMG_FORMAT_PPM_PARSER_H

#include "../image.h"

void write_ppm(const image& img, const char* filename);
image read_ppm(const char* filename);

#endif //LIB_IMAGE_IMG_FORMAT_PPM_PARSER_H