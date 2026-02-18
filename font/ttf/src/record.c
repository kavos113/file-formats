#include "record.h"

#include <stdlib.h>
#include <stdio.h>

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

  return directory;
}