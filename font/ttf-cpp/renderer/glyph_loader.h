#ifndef RENDERER_GLYPH_LOADER_H
#define RENDERER_GLYPH_LOADER_H

#include "tables/head.h"
#include "tables/maxp.h"
#include "ttf/types.h"


class GlyphLoader
{
public:
    GlyphLoader(const HeadTable& headTable, const MaxpTable& maxpTable);
    ~GlyphLoader() = default;

private:
    HeadTable headTable;
    MaxpTable maxpTable;

    // loca table
    std::vector<Offset32> glyphOffsets;
};


#endif //RENDERER_GLYPH_LOADER_H