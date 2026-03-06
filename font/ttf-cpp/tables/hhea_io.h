#ifndef TABLES_HHEA_IO_H
#define TABLES_HHEA_IO_H

#include <iostream>

#include "hhea.h"
#include "util/util.h"

template<typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, HHEATable& hhea)
{
    is.read(reinterpret_cast<char*>(&hhea), sizeof(HHEATable));

    endian_swap(&hhea.major_version, sizeof(uint16));
    endian_swap(&hhea.minor_version, sizeof(uint16));
    endian_swap(&hhea.ascender, sizeof(FWORD));
    endian_swap(&hhea.descender, sizeof(FWORD));
    endian_swap(&hhea.line_gap, sizeof(FWORD));
    endian_swap(&hhea.advance_width_max, sizeof(UFWORD));
    endian_swap(&hhea.min_left_side_bearing, sizeof(FWORD));
    endian_swap(&hhea.min_right_side_bearing, sizeof(FWORD));
    endian_swap(&hhea.x_max_extent, sizeof(FWORD));
    endian_swap(&hhea.caret_slope_rise, sizeof(int16));
    endian_swap(&hhea.caret_slope_run, sizeof(int16));
    endian_swap(&hhea.caret_offset, sizeof(int16));
    endian_swap(&hhea.metric_data_format, sizeof(int16));
    endian_swap(&hhea.number_of_h_metrics, sizeof(uint16));
    return is;
}

template<typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const HHEATable& hhea)
{
    os << "-----------------------------------------------------------------\n"
       << "Horizontal Header Table (hhea)\n"
       << "  majorVersion:        " << hhea.major_version << "\n"
       << "  minorVersion:        " << hhea.minor_version << "\n"
       << "  ascender:            " << hhea.ascender << "\n"
       << "  descender:           " << hhea.descender << "\n"
       << "  lineGap:             " << hhea.line_gap << "\n"
       << "  advanceWidthMax:     " << hhea.advance_width_max << "\n"
       << "  minLeftSideBearing:  " << hhea.min_left_side_bearing << "\n"
       << "  minRightSideBearing: " << hhea.min_right_side_bearing << "\n"
       << "  xMaxExtent:          " << hhea.x_max_extent << "\n"
       << "  caretSlopeRise:      " << hhea.caret_slope_rise << "\n"
       << "  caretSlopeRun:       " << hhea.caret_slope_run << "\n"
       << "  caretOffset:         " << hhea.caret_offset << "\n"
       << "  metricDataFormat:    " << hhea.metric_data_format << "\n"
       << "  numberOfHMetrics:    " << hhea.number_of_h_metrics << "\n";
    return os;
}

#endif //TABLES_HHEA_IO_H