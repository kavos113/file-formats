#include "record.h"

#include <stdlib.h>
#include <stdio.h>

void endian_swap(void *value, size_t size)
{
  uint8_t *bytes = (uint8_t *)value;
  for (size_t i = 0; i < size / 2; i++)
  {
    uint8_t temp = bytes[i];
    bytes[i] = bytes[size - 1 - i];
    bytes[size - 1 - i] = temp;
  }
}

TTFTableDirectory *read_ttf_table_directory(FILE *file)
{
  TTFTableDirectory *directory = malloc(sizeof(TTFTableDirectory));
  if (directory == NULL)
  {
    fprintf(stderr, "Failed to allocate memory for TTFTableDirectory\n");
    return NULL;
  }

  if (fread(&directory->sfnt_version, sizeof(uint32), 1, file) != 1 ||
      fread(&directory->num_tables, sizeof(uint16), 1, file) != 1 ||
      fread(&directory->search_range, sizeof(uint16), 1, file) != 1 ||
      fread(&directory->entry_selector, sizeof(uint16), 1, file) != 1 ||
      fread(&directory->range_shift, sizeof(uint16), 1, file) != 1)
  {
    fprintf(stderr, "Failed to read TTFTableDirectory header\n");
    free(directory);
    return NULL;
  }

  endian_swap(&directory->sfnt_version, sizeof(uint32));
  endian_swap(&directory->num_tables, sizeof(uint16));
  endian_swap(&directory->search_range, sizeof(uint16));
  endian_swap(&directory->entry_selector, sizeof(uint16));
  endian_swap(&directory->range_shift, sizeof(uint16));

  directory->table_records = malloc(sizeof(TTFTableRecord) * directory->num_tables);
  if (directory->table_records == NULL)
  {
    fprintf(stderr, "Failed to allocate memory for TTFTableRecord array\n");
    free(directory);
    return NULL;
  }

  for (int i = 0; i < directory->num_tables; i++)
  {
    if (fread(&directory->table_records[i], sizeof(TTFTableRecord), 1, file) != 1)
    {
      fprintf(stderr, "Failed to read TTFTableRecord at index %d\n", i);
      free(directory->table_records);
      free(directory);
      return NULL;
    }

    endian_swap(&directory->table_records[i].tag, sizeof(Tag));
    endian_swap(&directory->table_records[i].checksum, sizeof(uint32));
    endian_swap(&directory->table_records[i].offset, sizeof(Offset32));
    endian_swap(&directory->table_records[i].length, sizeof(uint32));
  }

  return directory;
}