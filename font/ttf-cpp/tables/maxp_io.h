#ifndef TABLES_MAXP_IO_H
#define TABLES_MAXP_IO_H

#include "maxp.h"
#include "util/util.h"

#include <iostream>

template<typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, MaxpTableVersion0_5& table)
{
    is.read(reinterpret_cast<char*>(&table), sizeof(MaxpTableVersion0_5));

    endian_swap(&table.version, sizeof(Version16Dot16));
    endian_swap(&table.num_glyphs, sizeof(uint16));
    return is;
}

template<typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const MaxpTableVersion0_5& table)
{
    os << "-----------------------------------------------------------------\n"
       << "Maxp Table (Version 0.5)\n"
       << "  Version: " << std::hex << table.version << std::dec << "\n"
       << "  Number of Glyphs: " << table.num_glyphs << "\n";
    return os;
}

template <typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, MaxpTableVersion1_0& table)
{
    is.read(reinterpret_cast<char*>(&table), sizeof(MaxpTableVersion1_0));

    endian_swap(&table.version, sizeof(Version16Dot16));
    endian_swap(&table.num_glyphs, sizeof(uint16));
    endian_swap(&table.max_points, sizeof(uint16));
    endian_swap(&table.max_contours, sizeof(uint16));
    endian_swap(&table.max_composite_points, sizeof(uint16));
    endian_swap(&table.max_composite_contours, sizeof(uint16));
    endian_swap(&table.max_zones, sizeof(uint16));
    endian_swap(&table.max_twilight_points, sizeof(uint16));
    endian_swap(&table.max_storage, sizeof(uint16));
    endian_swap(&table.max_function_defs, sizeof(uint16));
    endian_swap(&table.max_instruction_defs, sizeof(uint16));
    endian_swap(&table.max_stack_elements, sizeof(uint16));
    endian_swap(&table.max_size_of_instructions, sizeof(uint16));
    return is;
}

template <typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const MaxpTableVersion1_0& table)
{
    os << "-----------------------------------------------------------------\n"
       << "Maxp Table (Version 1.0)\n"
       << "  Version:                     " << std::hex << table.version << std::dec << "\n"
       << "  Number of Glyphs:            " << table.num_glyphs << "\n"
       << "  Max Points:                  " << table.max_points << "\n"
       << "  Max Contours:                " << table.max_contours << "\n"
       << "  Max Composite Points:        " << table.max_composite_points << "\n"
       << "  Max Composite Contours:      " << table.max_composite_contours << "\n"
       << "  Max Zones:                   " << table.max_zones << "\n"
       << "  Max Twilight Points:         " << table.max_twilight_points << "\n"
       << "  Max Storage:                 " << table.max_storage << "\n"
       << "  Max Function Definitions:    " << table.max_function_defs << "\n"
       << "  Max Instruction Definitions: " << table.max_instruction_defs << "\n"
       << "  Max Stack Elements:          " << table.max_stack_elements << "\n"
       << "  Max Size of Instructions:    " << table.max_size_of_instructions << "\n";
    return os;
}

enum MaxpVersion
{
    Version0_5 = 0x00005000,
    Version1_0 = 0x00010000
};

template <typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, MaxpTable& table)
{
    Version16Dot16 version;
    is.read(reinterpret_cast<char*>(&version), sizeof(Version16Dot16));
    endian_swap(&version, sizeof(Version16Dot16));

    is.seekg(-static_cast<std::streamoff>(sizeof(Version16Dot16)), std::ios::cur);

    if (version == Version0_5)
    {
        MaxpTableVersion0_5 maxpTable;
        is >> maxpTable;
        table = maxpTable;
    }
    else if (version == Version1_0)
    {
        MaxpTableVersion1_0 maxpTable;
        is >> maxpTable;
        table = maxpTable;
    }
    else
    {
        throw std::runtime_error("Unsupported maxp table version: " + std::to_string(version));
    }

    return is;
}

template <typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const MaxpTable& table)
{
    std::visit([&os](const auto& maxpTable) {
        os << maxpTable;
    }, table);
    return os;
}

#endif //TABLES_MAXP_IO_H