#include "glyph_loader.h"

GlyphLoader::GlyphLoader(const HeadTable& headTable, const MaxpTable& maxpTable)
    : headTable(headTable), maxpTable(maxpTable)
{
    if (headTable.indexToLocFormat == 0)
    {

    }
    else if (headTable.indexToLocFormat == 1)
    {

    }
}
