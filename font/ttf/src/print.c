#include "print.h"

void print_tag(Tag tag)
{
  printf("%c%c%c%c",
         (char)((tag >> 24) & 0xFF),
         (char)((tag >> 16) & 0xFF),
         (char)((tag >> 8) & 0xFF),
         (char)(tag & 0xFF));
}

void print_version16dot16(Version16Dot16 version)
{
  printf("%d.%d",
         (int)(version >> 16),
         (int)(version & 0xFFFF));
}

void print_ttf_table_directory(const TTFTableDirectory *directory)
{
  printf("SFNT Version:     0x%08x\n", directory->sfnt_version);
  printf("Number of Tables: %d\n", directory->num_tables);
  printf("Search Range:     %d\n", directory->search_range);
  printf("Entry Selector:   %d\n", directory->entry_selector);
  printf("Range Shift:      %d\n", directory->range_shift);

  printf("\nTable Records\n");
  printf("Tag   Checksum   Offset     Length    \n");
  // printf("aaaa  0x12345678 0x12345678 0x12345678\n");
  for (int i = 0; i < directory->num_tables; i++)
  {
    TTFTableRecord *record = &directory->table_records[i];
    print_tag(record->tag);
    printf(" 0x%08x 0x%08x 0x%08x\n", record->checksum, record->offset, record->length);
  }
}