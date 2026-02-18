#ifndef TTF_H
#define TTF_H

#include <stdint.h>

#include "ttftypes.h"

typedef struct _TTFTableRecord
{
  Tag tag;
  uint32 checksum;
  Offset32 offset;
  uint32 length;
} TTFTableRecord;

typedef struct _TTFTableDirectory
{
  uint32 sfnt_version;
  uint16 num_tables;
  uint16 search_range;
  uint16 entry_selector;
  uint16 range_shift;
  TTFTableRecord *table_records;
} TTFTableDirectory;

#endif // TTF_H