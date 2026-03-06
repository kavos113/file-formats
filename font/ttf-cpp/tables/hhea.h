#ifndef TABLES_HHEA_H
#define TABLES_HHEA_H

// horizontal header table

#include "ttf/types.h"

#pragma pack(push, 1)
struct HHEATable
{
    uint16 major_version;
    uint16 minor_version;
    FWORD ascender;
    FWORD descender;
    FWORD line_gap;
    UFWORD advance_width_max;
    FWORD min_left_side_bearing;
    FWORD min_right_side_bearing;
    FWORD x_max_extent;
    int16 caret_slope_rise;
    int16 caret_slope_run;
    int16 caret_offset;
    int16 reserved[4];
    int16 metric_data_format;
    uint16 number_of_h_metrics;
};
#pragma pack(pop)

#endif //TABLES_HHEA_H