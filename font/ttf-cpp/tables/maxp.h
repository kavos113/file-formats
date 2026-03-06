#ifndef TABLES_MAXP_H
#define TABLES_MAXP_H

#include "ttf/types.h"

#include <variant>

#pragma pack(push, 1)
struct MaxpTableVersion0_5
{
    Version16Dot16 version;
    uint16 num_glyphs;
};
#pragma pack(pop)

#pragma pack(push, 1)
struct MaxpTableVersion1_0
{
    Version16Dot16 version;
    uint16 num_glyphs;
    uint16 max_points;
    uint16 max_contours;
    uint16 max_composite_points;
    uint16 max_composite_contours;
    uint16 max_zones;
    uint16 max_twilight_points;
    uint16 max_storage;
    uint16 max_function_defs;
    uint16 max_instruction_defs;
    uint16 max_stack_elements;
    uint16 max_size_of_instructions;
};
#pragma pack(pop)

using MaxpTable = std::variant<MaxpTableVersion0_5, MaxpTableVersion1_0>;

#endif //TABLES_MAXP_H