#ifndef TABLES_HEAD_IO_H
#define TABLES_HEAD_IO_H

#include "head.h"

#include "util/util.h"

template<typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, HeadTable& head)
{
    is.read(reinterpret_cast<char*>(&head), sizeof(HeadTable));

    endian_swap(&head.majorVersion, sizeof(uint16));
    endian_swap(&head.minorVersion, sizeof(uint16));
    endian_swap(&head.fontRevision, sizeof(Fixed));
    endian_swap(&head.checksumAdjustment, sizeof(uint32));
    endian_swap(&head.magicNumber, sizeof(uint32));
    endian_swap(&head.flags, sizeof(uint16));
    endian_swap(&head.unitsPerEm, sizeof(uint16));
    endian_swap(&head.created, sizeof(LONGDATETIME));
    endian_swap(&head.modified, sizeof(LONGDATETIME));
    endian_swap(&head.xMin, sizeof(int16));
    endian_swap(&head.yMin, sizeof(int16));
    endian_swap(&head.xMax, sizeof(int16));
    endian_swap(&head.yMax, sizeof(int16));
    endian_swap(&head.macStyle, sizeof(uint16));
    endian_swap(&head.lowestRecPPEM, sizeof(uint16));
    endian_swap(&head.fontDirectionHint, sizeof(int16));
    endian_swap(&head.indexToLocFormat, sizeof(int16));
    endian_swap(&head.glyphDataFormat, sizeof(int16));

    return is;
}

enum class MacStyleFlags : uint16
{
    Bold      = 1 << 0,
    Italic    = 1 << 1,
    Underline = 1 << 2,
    Outline   = 1 << 3,
    Shadow    = 1 << 4,
    Condensed = 1 << 5,
    Extended  = 1 << 6
};

inline std::string get_mac_style_string(uint16 macStyle)
{
    std::stringstream ss;
    if (macStyle & static_cast<uint16>(MacStyleFlags::Bold))
        ss << "Bold ";
    if (macStyle & static_cast<uint16>(MacStyleFlags::Italic))
        ss << "Italic ";
    if (macStyle & static_cast<uint16>(MacStyleFlags::Underline))
        ss << "Underline ";
    if (macStyle & static_cast<uint16>(MacStyleFlags::Outline))
        ss << "Outline ";
    if (macStyle & static_cast<uint16>(MacStyleFlags::Shadow))
        ss << "Shadow ";
    if (macStyle & static_cast<uint16>(MacStyleFlags::Condensed))
        ss << "Condensed ";
    if (macStyle & static_cast<uint16>(MacStyleFlags::Extended))
        ss << "Extended ";

    return ss.str();
}

inline std::string flag_string(uint16 flags)
{
    std::stringstream ss;

    if (flags & 0x0001) ss << "    - Baseline at y=0\n";
    if (flags & 0x0002) ss << "    - Left sidebearing at x=0 \n";
    if (flags & 0x0004) ss << "    - Instructions may depend on point size and ppem \n";
    if (flags & 0x0008) ss << "    - Force ppem to integer values for all internal scaler math\n";
    if (flags & 0x0010) ss << "    - Instructions may alter advance width\n";
    if (flags & 0x0800) ss << "    - Font data is 'lossless' (e.g. for CFF outlines) \n";
    if (flags & 0x1000) ss << "    - Font converted (produce compatible metrics) \n";
    if (flags & 0x2000) ss << "    - Font optimized for ClearType\n";
    if (flags & 0x4000) ss << "    - Last Resort font\n";

    return ss.str();
}

template<typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const HeadTable& head)
{
    os << "\n-----------------------------------------------------------------\n"
       <<  "Head Table\n"
       << "  majorVersion:            " << head.majorVersion << "\n"
       << "  minorVersion:            " << head.minorVersion << "\n"
       << "  fontRevision:            " << std::hex << "0x" << head.fontRevision << std::dec << "\n"
       << "  checksumAdjustment:      " << std::hex << "0x" << head.checksumAdjustment << std::dec << "\n"
       << "  magicNumber:             " << std::hex << "0x" << head.magicNumber << std::dec << "\n"
       << "  unitsPerEm:              " << head.unitsPerEm << "\n"
       << "  created:                 " << longdatetime_to_string(head.created) << "\n"
       << "  modified:                " << longdatetime_to_string(head.modified) << "\n"
       << "  xMin:                    " << head.xMin << "\n"
       << "  yMin:                    " << head.yMin << "\n"
       << "  xMax:                    " << head.xMax << "\n"
       << "  yMax:                    " << head.yMax << "\n"
       << "  macStyle:                " << get_mac_style_string(head.macStyle) << "\n"
       << "  lowestRecPPEM:           " << head.lowestRecPPEM << "\n"
       << "  fontDirectionHint:       " << head.fontDirectionHint << "\n"
       << "  indexToLocFormat:        " << (head.indexToLocFormat == 0 ? "Short offsets (uint16)" : "Long offsets (uint32)") << "\n"
       << "  glyphDataFormat (set 0): " << head.glyphDataFormat << "\n"
       << "  flags:                   \n" << flag_string(head.flags) << "\n";

    os <<"-----------------------------------------------------------------\n";

    return os;
}

#endif //TABLES_HEAD_IO_H