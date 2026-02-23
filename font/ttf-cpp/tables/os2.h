#ifndef TABLES_OS2_H
#define TABLES_OS2_H

#include "ttf/types.h"

#include <iostream>
#include <variant>

#pragma pack(push, 1)
struct OS2_V5
{
    uint16 version;
    FWORD xAvgCharWidth;
    uint16 usWeightClass;
    uint16 usWidthClass;
    uint16 fsType;
    FWORD ySubscriptXSize;
    FWORD ySubscriptYSize;
    FWORD ySubscriptXOffset;
    FWORD ySubscriptYOffset;
    FWORD ySuperscriptXSize;
    FWORD ySuperscriptYSize;
    FWORD ySuperscriptXOffset;
    FWORD ySuperscriptYOffset;
    FWORD yStrikeoutSize;
    FWORD yStrikeoutPosition;
    int16 sFamilyClass;
    uint8 panose[10];
    uint32 ulUnicodeRange1;
    uint32 ulUnicodeRange2;
    uint32 ulUnicodeRange3;
    uint32 ulUnicodeRange4;
    Tag achVendID;
    uint16 fsSelection;
    uint16 usFirstCharIndex;
    uint16 usLastCharIndex;
    FWORD sTypoAscender;
    FWORD sTypoDescender;
    FWORD sTypoLineGap;
    UFWORD usWinAscent;
    UFWORD usWinDescent;
    uint32 ulCodePageRange1;
    uint32 ulCodePageRange2;
    FWORD sxHeight;
    FWORD sCapHeight;
    uint16 usDefaultChar;
    uint16 usBreakChar;
    uint16 usMaxContext;
    uint16 usLowerOpticalPointSize;
    uint16 usUpperOpticalPointSize;
};
#pragma pack(pop)

// v2, v3 are the same as v4
#pragma pack(push, 1)
struct OS2_V4
{
    uint16 version;
    FWORD xAvgCharWidth;
    uint16 usWeightClass;
    uint16 usWidthClass;
    uint16 fsType;
    FWORD ySubscriptXSize;
    FWORD ySubscriptYSize;
    FWORD ySubscriptXOffset;
    FWORD ySubscriptYOffset;
    FWORD ySuperscriptXSize;
    FWORD ySuperscriptYSize;
    FWORD ySuperscriptXOffset;
    FWORD ySuperscriptYOffset;
    FWORD yStrikeoutSize;
    FWORD yStrikeoutPosition;
    int16 sFamilyClass;
    uint8 panose[10];
    uint32 ulUnicodeRange1;
    uint32 ulUnicodeRange2;
    uint32 ulUnicodeRange3;
    uint32 ulUnicodeRange4;
    Tag achVendID;
    uint16 fsSelection;
    uint16 usFirstCharIndex;
    uint16 usLastCharIndex;
    FWORD sTypoAscender;
    FWORD sTypoDescender;
    FWORD sTypoLineGap;
    UFWORD usWinAscent;
    UFWORD usWinDescent;
    uint32 ulCodePageRange1;
    uint32 ulCodePageRange2;
    FWORD sxHeight;
    FWORD sCapHeight;
    uint16 usDefaultChar;
    uint16 usBreakChar;
    uint16 usMaxContext;
};
#pragma pack(pop)

#pragma pack(push, 1)
struct OS2_V1
{
    uint16 version;
    FWORD xAvgCharWidth;
    uint16 usWeightClass;
    uint16 usWidthClass;
    uint16 fsType;
    FWORD ySubscriptXSize;
    FWORD ySubscriptYSize;
    FWORD ySubscriptXOffset;
    FWORD ySubscriptYOffset;
    FWORD ySuperscriptXSize;
    FWORD ySuperscriptYSize;
    FWORD ySuperscriptXOffset;
    FWORD ySuperscriptYOffset;
    FWORD yStrikeoutSize;
    FWORD yStrikeoutPosition;
    int16 sFamilyClass;
    uint8 panose[10];
    uint32 ulUnicodeRange1;
    uint32 ulUnicodeRange2;
    uint32 ulUnicodeRange3;
    uint32 ulUnicodeRange4;
    Tag achVendID;
    uint16 fsSelection;
    uint16 usFirstCharIndex;
    uint16 usLastCharIndex;
    FWORD sTypoAscender;
    FWORD sTypoDescender;
    FWORD sTypoLineGap;
    UFWORD usWinAscent;
    UFWORD usWinDescent;
    uint32 ulCodePageRange1;
    uint32 ulCodePageRange2;
};
#pragma pack(pop)

#pragma pack(push, 1)
struct OS2_V0
{
    uint16 version;
    FWORD xAvgCharWidth;
    uint16 usWeightClass;
    uint16 usWidthClass;
    uint16 fsType;
    FWORD ySubscriptXSize;
    FWORD ySubscriptYSize;
    FWORD ySubscriptXOffset;
    FWORD ySubscriptYOffset;
    FWORD ySuperscriptXSize;
    FWORD ySuperscriptYSize;
    FWORD ySuperscriptXOffset;
    FWORD ySuperscriptYOffset;
    FWORD yStrikeoutSize;
    FWORD yStrikeoutPosition;
    int16 sFamilyClass;
    uint8 panose[10];
    uint32 ulUnicodeRange1;
    uint32 ulUnicodeRange2;
    uint32 ulUnicodeRange3;
    uint32 ulUnicodeRange4;
    Tag achVendID;
    uint16 fsSelection;
    uint16 usFirstCharIndex;
    uint16 usLastCharIndex;
    FWORD sTypoAscender;
    FWORD sTypoDescender;
    FWORD sTypoLineGap;
    UFWORD usWinAscent;
    UFWORD usWinDescent;
};
#pragma pack(pop)

using OS2Table = std::variant<OS2_V0, OS2_V1, OS2_V4, OS2_V5>;

#endif // TABLES_OS2_H