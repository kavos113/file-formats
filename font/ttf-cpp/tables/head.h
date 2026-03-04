#ifndef TABLES_HEAD_H
#define TABLES_HEAD_H

#include "ttf/types.h"

#include <iostream>

#pragma pack(push, 1)
struct HeadTable
{
    uint16 majorVersion;
    uint16 minorVersion;
    Fixed fontRevision;
    uint32 checksumAdjustment;
    uint32 magicNumber;
    uint16 flags;
    uint16 unitsPerEm;
    LONGDATETIME created;
    LONGDATETIME modified;
    int16 xMin;
    int16 yMin;
    int16 xMax;
    int16 yMax;
    uint16 macStyle;
    uint16 lowestRecPPEM;
    int16 fontDirectionHint;
    int16 indexToLocFormat;
    int16 glyphDataFormat;
};
#pragma pack(pop)

template<typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, HeadTable& head);

template<typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const HeadTable& head);

#endif //TABLES_HEAD_H