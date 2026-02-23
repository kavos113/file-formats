#ifndef TABLES_OS2_IO_H
#define TABLES_OS2_IO_H

#include "os2.h"

#include "util/util.h"
#include <format>

template<typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, OS2Table& os2)
{
    uint16 version;
    is.read(reinterpret_cast<char*>(&version), sizeof(uint16));
    is.seekg(-static_cast<std::streamoff>(sizeof(uint16)), std::ios::cur);

    endian_swap(&version, sizeof(uint16));

    switch (version)
    {
    case 0:
        os2 = OS2_V0{};
        return is >> std::get<OS2_V0>(os2);
    case 1:
        os2 = OS2_V1{};
        return is >> std::get<OS2_V1>(os2);
    case 2:
    case 3:
    case 4:
        os2 = OS2_V4{};
        return is >> std::get<OS2_V4>(os2);
    case 5:
        os2 = OS2_V5{};
        return is >> std::get<OS2_V5>(os2);

    default:
        throw std::runtime_error("Unsupported OS/2 table version: " + std::to_string(version));
    }
}

template <typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const OS2Table& os2)
{
    os << "\n-----------------------------------------------------------------\n"
       <<  std::format("OS/2 Table (version {})\n", std::visit([](const auto& os2) { return os2.version; }, os2));

    std::visit([&os](const auto& os2) { os << os2; }, os2);

    os << "-----------------------------------------------------------------\n";

    return os;
}

template <typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, OS2_V0& os2)
{
    is.read(reinterpret_cast<char*>(&os2), sizeof(OS2_V0));

    endian_swap(&os2.version, sizeof(uint16));
    endian_swap(&os2.xAvgCharWidth, sizeof(FWORD));
    endian_swap(&os2.usWeightClass, sizeof(uint16));
    endian_swap(&os2.usWidthClass, sizeof(uint16));
    endian_swap(&os2.fsType, sizeof(uint16));
    endian_swap(&os2.ySubscriptXSize, sizeof(FWORD));
    endian_swap(&os2.ySubscriptYSize, sizeof(FWORD));
    endian_swap(&os2.ySubscriptXOffset, sizeof(FWORD));
    endian_swap(&os2.ySubscriptYOffset, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptXSize, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptYSize, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptXOffset, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptYOffset, sizeof(FWORD));
    endian_swap(&os2.yStrikeoutSize, sizeof(FWORD));
    endian_swap(&os2.yStrikeoutPosition, sizeof(FWORD));
    endian_swap(&os2.sFamilyClass, sizeof(int16));
    endian_swap(&os2.ulUnicodeRange1, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange2, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange3, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange4, sizeof(uint32));
    endian_swap(&os2.achVendID, sizeof(Tag));
    endian_swap(&os2.fsSelection, sizeof(uint16));
    endian_swap(&os2.usFirstCharIndex, sizeof(uint16));
    endian_swap(&os2.usLastCharIndex, sizeof(uint16));
    endian_swap(&os2.sTypoAscender, sizeof(FWORD));
    endian_swap(&os2.sTypoDescender, sizeof(FWORD));
    endian_swap(&os2.sTypoLineGap, sizeof(FWORD));
    endian_swap(&os2.usWinAscent, sizeof(UFWORD));
    endian_swap(&os2.usWinDescent, sizeof(UFWORD));

    return is;
}

template <typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const OS2_V0& os2)
{
    os << "  version:             " << os2.version << "\n"
       << "  xAvgCharWidth:       " << os2.xAvgCharWidth << "\n"
       << "  usWeightClass:       " << os2.usWeightClass << "\n"
       << "  usWidthClass:        " << os2.usWidthClass << "\n"
       << "  fsType:              " << os2.fsType << "\n"
       << "  ySubscriptXSize:     " << os2.ySubscriptXSize << "\n"
       << "  ySubscriptYSize:     " << os2.ySubscriptYSize << "\n"
       << "  ySubscriptXOffset:   " << os2.ySubscriptXOffset << "\n"
       << "  ySubscriptYOffset:   " << os2.ySubscriptYOffset << "\n"
       << "  ySuperscriptXSize:   " << os2.ySuperscriptXSize << "\n"
       << "  ySuperscriptYSize:   " << os2.ySuperscriptYSize << "\n"
       << "  ySuperscriptXOffset: " << os2.ySuperscriptXOffset << "\n"
       << "  ySuperscriptYOffset: " << os2.ySuperscriptYOffset << "\n"
       << "  yStrikeoutSize:      " << os2.yStrikeoutSize << "\n"
       << "  yStrikeoutPosition:  " << os2.yStrikeoutPosition << "\n"
       << "  sFamilyClass:        " << os2.sFamilyClass << "\n"
       << "  panose:              "
       << std::format("0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X}\n",
           os2.panose[0], os2.panose[1], os2.panose[2], os2.panose[3], os2.panose[4],
           os2.panose[5], os2.panose[6], os2.panose[7], os2.panose[8], os2.panose[9])
       << "  ulUnicodeRange1:     " << os2.ulUnicodeRange1 << "\n"
       << "  ulUnicodeRange2:     " << os2.ulUnicodeRange2 << "\n"
       << "  ulUnicodeRange3:     " << os2.ulUnicodeRange3 << "\n"
       << "  ulUnicodeRange4:     " << os2.ulUnicodeRange4 << "\n"
       << "  achVendID:           " << tag_to_string(os2.achVendID) << "\n"
       << "  fsSelection:         " << os2.fsSelection << "\n"
       << "  usFirstCharIndex:    " << os2.usFirstCharIndex << "\n"
       << "  usLastCharIndex:     " << os2.usLastCharIndex << "\n"
       << "  sTypoAscender:       " << os2.sTypoAscender << "\n"
       << "  sTypoDescender:      " << os2.sTypoDescender << "\n"
       << "  sTypoLineGap:        " << os2.sTypoLineGap << "\n"
       << "  usWinAscent:         " << os2.usWinAscent << "\n"
       << "  usWinDescent:        " << os2.usWinDescent << "\n";
     return os;
}

template <typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, OS2_V1& os2)
{
    is.read(reinterpret_cast<char*>(&os2), sizeof(OS2_V1));

    endian_swap(&os2.version, sizeof(uint16));
    endian_swap(&os2.xAvgCharWidth, sizeof(FWORD));
    endian_swap(&os2.usWeightClass, sizeof(uint16));
    endian_swap(&os2.usWidthClass, sizeof(uint16));
    endian_swap(&os2.fsType, sizeof(uint16));
    endian_swap(&os2.ySubscriptXSize, sizeof(FWORD));
    endian_swap(&os2.ySubscriptYSize, sizeof(FWORD));
    endian_swap(&os2.ySubscriptXOffset, sizeof(FWORD));
    endian_swap(&os2.ySubscriptYOffset, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptXSize, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptYSize, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptXOffset, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptYOffset, sizeof(FWORD));
    endian_swap(&os2.yStrikeoutSize, sizeof(FWORD));
    endian_swap(&os2.yStrikeoutPosition, sizeof(FWORD));
    endian_swap(&os2.sFamilyClass, sizeof(int16));
    endian_swap(&os2.ulUnicodeRange1, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange2, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange3, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange4, sizeof(uint32));
    endian_swap(&os2.achVendID, sizeof(Tag));
    endian_swap(&os2.fsSelection, sizeof(uint16));
    endian_swap(&os2.usFirstCharIndex, sizeof(uint16));
    endian_swap(&os2.usLastCharIndex, sizeof(uint16));
    endian_swap(&os2.sTypoAscender, sizeof(FWORD));
    endian_swap(&os2.sTypoDescender, sizeof(FWORD));
    endian_swap(&os2.sTypoLineGap, sizeof(FWORD));
    endian_swap(&os2.usWinAscent, sizeof(UFWORD));
    endian_swap(&os2.usWinDescent, sizeof(UFWORD));
    endian_swap(&os2.ulCodePageRange1, sizeof(uint32));
    endian_swap(&os2.ulCodePageRange2, sizeof(uint32));

    return is;
}

template <typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const OS2_V1& os2)
{
    os << "  version:             " << os2.version << "\n"
       << "  xAvgCharWidth:       " << os2.xAvgCharWidth << "\n"
       << "  usWeightClass:       " << os2.usWeightClass << "\n"
       << "  usWidthClass:        " << os2.usWidthClass << "\n"
       << "  fsType:              " << os2.fsType << "\n"
       << "  ySubscriptXSize:     " << os2.ySubscriptXSize << "\n"
       << "  ySubscriptYSize:     " << os2.ySubscriptYSize << "\n"
       << "  ySubscriptXOffset:   " << os2.ySubscriptXOffset << "\n"
       << "  ySubscriptYOffset:   " << os2.ySubscriptYOffset << "\n"
       << "  ySuperscriptXSize:   " << os2.ySuperscriptXSize << "\n"
       << "  ySuperscriptYSize:   " << os2.ySuperscriptYSize << "\n"
       << "  ySuperscriptXOffset: " << os2.ySuperscriptXOffset << "\n"
       << "  ySuperscriptYOffset: " << os2.ySuperscriptYOffset << "\n"
       << "  yStrikeoutSize:      " << os2.yStrikeoutSize << "\n"
       << "  yStrikeoutPosition:  " << os2.yStrikeoutPosition << "\n"
       << "  sFamilyClass:        " << os2.sFamilyClass << "\n"
       << "  panose:              "
       << std::format("0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X}\n",
           os2.panose[0], os2.panose[1], os2.panose[2], os2.panose[3], os2.panose[4],
           os2.panose[5], os2.panose[6], os2.panose[7], os2.panose[8], os2.panose[9])
       << "  ulUnicodeRange1:     " << os2.ulUnicodeRange1 << "\n"
       << "  ulUnicodeRange2:     " << os2.ulUnicodeRange2 << "\n"
       << "  ulUnicodeRange3:     " << os2.ulUnicodeRange3 << "\n"
       << "  ulUnicodeRange4:     " << os2.ulUnicodeRange4 << "\n"
       << "  achVendID:           " << tag_to_string(os2.achVendID) << "\n"
       << "  fsSelection:         " << os2.fsSelection << "\n"
       << "  usFirstCharIndex:    " << os2.usFirstCharIndex << "\n"
       << "  usLastCharIndex:     " << os2.usLastCharIndex << "\n"
       << "  sTypoAscender:       " << os2.sTypoAscender << "\n"
       << "  sTypoDescender:      " << os2.sTypoDescender << "\n"
       << "  sTypoLineGap:        " << os2.sTypoLineGap << "\n"
       << "  usWinAscent:         " << os2.usWinAscent << "\n"
       << "  usWinDescent:        " << os2.usWinDescent << "\n"
       << "  ulCodePageRange1:    " << os2.ulCodePageRange1 << "\n"
       << "  ulCodePageRange2:    " << os2.ulCodePageRange2 << "\n";
     return os;
}

template <typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, OS2_V4& os2)
{
    is.read(reinterpret_cast<char*>(&os2), sizeof(OS2_V4));

    endian_swap(&os2.version, sizeof(uint16));
    endian_swap(&os2.xAvgCharWidth, sizeof(FWORD));
    endian_swap(&os2.usWeightClass, sizeof(uint16));
    endian_swap(&os2.usWidthClass, sizeof(uint16));
    endian_swap(&os2.fsType, sizeof(uint16));
    endian_swap(&os2.ySubscriptXSize, sizeof(FWORD));
    endian_swap(&os2.ySubscriptYSize, sizeof(FWORD));
    endian_swap(&os2.ySubscriptXOffset, sizeof(FWORD));
    endian_swap(&os2.ySubscriptYOffset, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptXSize, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptYSize, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptXOffset, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptYOffset, sizeof(FWORD));
    endian_swap(&os2.yStrikeoutSize, sizeof(FWORD));
    endian_swap(&os2.yStrikeoutPosition, sizeof(FWORD));
    endian_swap(&os2.sFamilyClass, sizeof(int16));
    endian_swap(&os2.ulUnicodeRange1, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange2, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange3, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange4, sizeof(uint32));
    endian_swap(&os2.achVendID, sizeof(Tag));
    endian_swap(&os2.fsSelection, sizeof(uint16));
    endian_swap(&os2.usFirstCharIndex, sizeof(uint16));
    endian_swap(&os2.usLastCharIndex, sizeof(uint16));
    endian_swap(&os2.sTypoAscender, sizeof(FWORD));
    endian_swap(&os2.sTypoDescender, sizeof(FWORD));
    endian_swap(&os2.sTypoLineGap, sizeof(FWORD));
    endian_swap(&os2.usWinAscent, sizeof(UFWORD));
    endian_swap(&os2.usWinDescent, sizeof(UFWORD));
    endian_swap(&os2.ulCodePageRange1, sizeof(uint32));
    endian_swap(&os2.ulCodePageRange2, sizeof(uint32));
    endian_swap(&os2.sxHeight, sizeof(FWORD));
    endian_swap(&os2.sCapHeight, sizeof(FWORD));
    endian_swap(&os2.usDefaultChar, sizeof(uint16));
    endian_swap(&os2.usBreakChar, sizeof(uint16));
    endian_swap(&os2.usMaxContext, sizeof(uint16));

    return is;
}

template <typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const OS2_V4& os2)
{
    os << "  version:             " << os2.version << "\n"
       << "  xAvgCharWidth:       " << os2.xAvgCharWidth << "\n"
       << "  usWeightClass:       " << os2.usWeightClass << "\n"
       << "  usWidthClass:        " << os2.usWidthClass << "\n"
       << "  fsType:              " << os2.fsType << "\n"
       << "  ySubscriptXSize:     " << os2.ySubscriptXSize << "\n"
       << "  ySubscriptYSize:     " << os2.ySubscriptYSize << "\n"
       << "  ySubscriptXOffset:   " << os2.ySubscriptXOffset << "\n"
       << "  ySubscriptYOffset:   " << os2.ySubscriptYOffset << "\n"
       << "  ySuperscriptXSize:   " << os2.ySuperscriptXSize << "\n"
       << "  ySuperscriptYSize:   " << os2.ySuperscriptYSize << "\n"
       << "  ySuperscriptXOffset: " << os2.ySuperscriptXOffset << "\n"
       << "  ySuperscriptYOffset: " << os2.ySuperscriptYOffset << "\n"
       << "  yStrikeoutSize:      " << os2.yStrikeoutSize << "\n"
       << "  yStrikeoutPosition:  " << os2.yStrikeoutPosition << "\n"
       << "  sFamilyClass:        " << os2.sFamilyClass << "\n"
       << "  panose:              "
       << std::format("0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X}\n",
           os2.panose[0], os2.panose[1], os2.panose[2], os2.panose[3], os2.panose[4],
           os2.panose[5], os2.panose[6], os2.panose[7], os2.panose[8], os2.panose[9])
       << "  ulUnicodeRange1:     " << os2.ulUnicodeRange1 << "\n"
       << "  ulUnicodeRange2:     " << os2.ulUnicodeRange2 << "\n"
       << "  ulUnicodeRange3:     " << os2.ulUnicodeRange3 << "\n"
       << "  ulUnicodeRange4:     " << os2.ulUnicodeRange4 << "\n"
       << "  achVendID:           " << tag_to_string(os2.achVendID) << "\n"
       << "  fsSelection:         " << os2.fsSelection << "\n"
       << "  usFirstCharIndex:    " << os2.usFirstCharIndex << "\n"
       << "  usLastCharIndex:     " << os2.usLastCharIndex << "\n"
       << "  sTypoAscender:       " << os2.sTypoAscender << "\n"
       << "  sTypoDescender:      " << os2.sTypoDescender << "\n"
       << "  sTypoLineGap:        " << os2.sTypoLineGap << "\n"
       << "  usWinAscent:         " << os2.usWinAscent << "\n"
       << "  usWinDescent:        " << os2.usWinDescent << "\n"
       << "  ulCodePageRange1:    " << os2.ulCodePageRange1 << "\n"
       << "  ulCodePageRange2:    " << os2.ulCodePageRange2 << "\n"
       << "  sxHeight:            " << os2.sxHeight << "\n"
       << "  sCapHeight:          " << os2.sCapHeight << "\n"
       << "  usDefaultChar:       " << os2.usDefaultChar << "\n"
       << "  usBreakChar:         " << os2.usBreakChar << "\n"
       << "  usMaxContext:        " << os2.usMaxContext << "\n";

    return os;
}

template <typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, OS2_V5& os2)
{
    is.read(reinterpret_cast<char*>(&os2), sizeof(OS2_V5));

    endian_swap(&os2.version, sizeof(uint16));
    endian_swap(&os2.xAvgCharWidth, sizeof(FWORD));
    endian_swap(&os2.usWeightClass, sizeof(uint16));
    endian_swap(&os2.usWidthClass, sizeof(uint16));
    endian_swap(&os2.fsType, sizeof(uint16));
    endian_swap(&os2.ySubscriptXSize, sizeof(FWORD));
    endian_swap(&os2.ySubscriptYSize, sizeof(FWORD));
    endian_swap(&os2.ySubscriptXOffset, sizeof(FWORD));
    endian_swap(&os2.ySubscriptYOffset, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptXSize, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptYSize, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptXOffset, sizeof(FWORD));
    endian_swap(&os2.ySuperscriptYOffset, sizeof(FWORD));
    endian_swap(&os2.yStrikeoutSize, sizeof(FWORD));
    endian_swap(&os2.yStrikeoutPosition, sizeof(FWORD));
    endian_swap(&os2.sFamilyClass, sizeof(int16));
    endian_swap(&os2.ulUnicodeRange1, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange2, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange3, sizeof(uint32));
    endian_swap(&os2.ulUnicodeRange4, sizeof(uint32));
    endian_swap(&os2.achVendID, sizeof(Tag));
    endian_swap(&os2.fsSelection, sizeof(uint16));
    endian_swap(&os2.usFirstCharIndex, sizeof(uint16));
    endian_swap(&os2.usLastCharIndex, sizeof(uint16));
    endian_swap(&os2.sTypoAscender, sizeof(FWORD));
    endian_swap(&os2.sTypoDescender, sizeof(FWORD));
    endian_swap(&os2.sTypoLineGap, sizeof(FWORD));
    endian_swap(&os2.usWinAscent, sizeof(UFWORD));
    endian_swap(&os2.usWinDescent, sizeof(UFWORD));
    endian_swap(&os2.ulCodePageRange1, sizeof(uint32));
    endian_swap(&os2.ulCodePageRange2, sizeof(uint32));
    endian_swap(&os2.sxHeight, sizeof(FWORD));
    endian_swap(&os2.sCapHeight, sizeof(FWORD));
    endian_swap(&os2.usDefaultChar, sizeof(uint16));
    endian_swap(&os2.usBreakChar, sizeof(uint16));
    endian_swap(&os2.usMaxContext, sizeof(uint16));
    endian_swap(&os2.usLowerOpticalPointSize, sizeof(uint16));
    endian_swap(&os2.usUpperOpticalPointSize, sizeof(uint16));

    return is;
}

template <typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const OS2_V5& os2)
{
    os << "  version:             " << os2.version << "\n"
       << "  xAvgCharWidth:       " << os2.xAvgCharWidth << "\n"
       << "  usWeightClass:       " << os2.usWeightClass << "\n"
       << "  usWidthClass:        " << os2.usWidthClass << "\n"
       << "  fsType:              " << os2.fsType << "\n"
       << "  ySubscriptXSize:     " << os2.ySubscriptXSize << "\n"
       << "  ySubscriptYSize:     " << os2.ySubscriptYSize << "\n"
       << "  ySubscriptXOffset:   " << os2.ySubscriptXOffset << "\n"
       << "  ySubscriptYOffset:   " << os2.ySubscriptYOffset << "\n"
       << "  ySuperscriptXSize:   " << os2.ySuperscriptXSize << "\n"
       << "  ySuperscriptYSize:   " << os2.ySuperscriptYSize << "\n"
       << "  ySuperscriptXOffset: " << os2.ySuperscriptXOffset << "\n"
       << "  ySuperscriptYOffset: " << os2.ySuperscriptYOffset << "\n"
       << "  yStrikeoutSize:      " << os2.yStrikeoutSize << "\n"
       << "  yStrikeoutPosition:  " << os2.yStrikeoutPosition << "\n"
       << "  sFamilyClass:        " << os2.sFamilyClass << "\n"
       << "  panose:              "
       << std::format("0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X}\n",
           os2.panose[0], os2.panose[1], os2.panose[2], os2.panose[3], os2.panose[4],
           os2.panose[5], os2.panose[6], os2.panose[7], os2.panose[8], os2.panose[9])
       << "  ulUnicodeRange1:     " << os2.ulUnicodeRange1 << "\n"
       << "  ulUnicodeRange2:     " << os2.ulUnicodeRange2 << "\n"
       << "  ulUnicodeRange3:     " << os2.ulUnicodeRange3 << "\n"
       << "  ulUnicodeRange4:     " << os2.ulUnicodeRange4 << "\n"
       << "  achVendID:           " << tag_to_string(os2.achVendID) << "\n"
       << "  fsSelection:         " << os2.fsSelection << "\n"
       << "  usFirstCharIndex:    " << os2.usFirstCharIndex << "\n"
       << "  usLastCharIndex:     " << os2.usLastCharIndex << "\n"
       << "  sTypoAscender:       " << os2.sTypoAscender << "\n"
       << "  sTypoDescender:      " << os2.sTypoDescender << "\n"
       << "  sTypoLineGap:        " << os2.sTypoLineGap << "\n"
       << "  usWinAscent:         " << os2.usWinAscent << "\n"
       << "  usWinDescent:        " << os2.usWinDescent << "\n"
       << "  ulCodePageRange1:    " << os2.ulCodePageRange1 << "\n"
       << "  ulCodePageRange2:    " << os2.ulCodePageRange2 << "\n"
       << "  sxHeight:            " << os2.sxHeight << "\n"
       << "  sCapHeight:          " << os2.sCapHeight << "\n"
       << "  usDefaultChar:       " << os2.usDefaultChar << "\n"
       << "  usBreakChar:         " << os2.usBreakChar << "\n"
       << "  usMaxContext:        " << os2.usMaxContext << "\n"
       << "  usLowerOpticalPointSize: " << os2.usLowerOpticalPointSize << "\n"
       << "  usUpperOpticalPointSize: " << os2.usUpperOpticalPointSize << "\n";

    return os;
}


#endif // TABLES_OS2_IO_H