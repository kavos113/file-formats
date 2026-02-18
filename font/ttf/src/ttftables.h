#ifndef TTFTABLES_H
#define TTFTABLES_H

#include "ttftypes.h"

// -------- 'cmap' table --------
typedef struct _CMAPEncodingRecord
{
  uint16 platform_id;
  uint16 encoding_id;
  Offset32 subtable_offset;
} CMAPEncodingRecord;

typedef struct _CMAPHeader
{
  uint16 version;
  uint16 num_tables;
  CMAPEncodingRecord *encoding_records;
} CMAPHeader;

#define CMAP_PLATFORM_ID_UNICORD 0
#define CMAP_PLATFORM_ID_MACINTOSH 1
#define CMAP_PLATFORM_ID_ISO 2
#define CMAP_PLATFORM_ID_WINDOWS 3
#define CMAP_PLATFORM_ID_CUSTOM 4

#define CMAP_ENCODING_ID_UNICORD_1_0 0
#define CMAP_ENCODING_ID_UNICORD_1_1 1
#define CMAP_ENCODING_ID_UNICORD_ISO 2
#define CMAP_ENCODING_ID_UNICORD_2_0_BMP 3
#define CMAP_ENCODING_ID_UNICORD_2_0_FULL 4
#define CMAP_ENCODING_ID_UNICORD_VARIATION_SEQUENCES 5
#define CMAP_ENCODING_ID_UNICORD_FULL_REPERTOIRE 6
#define CMAP_ENCODING_ID_WINDOWS_SYMBOL 0
#define CMAP_ENCODING_ID_WINDOWS_UNICORD_BMP 1
#define CMAP_ENCODING_ID_WINDOWS_SHIFTJIS 2
#define CMAP_ENCODING_ID_WINDOWS_PRC 3
#define CMAP_ENCODING_ID_WINDOWS_BIG5 4
#define CMAP_ENCODING_ID_WINDOWS_WANSUNG 5
#define CMAP_ENCODING_ID_WINDOWS_JOHAB 6
#define CMAP_ENCODING_ID_WINDOWS_UNICORD_FULL 10

typedef struct _CMAPSubtableHeader
{
  uint16 format;
  uint16 length;
  uint16 language; // only mac, lang ID + 1
} CMAPSubtableHeader;

#endif // TTFTABLES_H