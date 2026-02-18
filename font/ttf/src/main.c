#include "record.h"
#include "ttf.h"
#include "ttftypes.h"
#include "print.h"

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
  if (argc != 2)
  {
    fprintf(stderr, "usage: readttf <ttf file>\n");
    return 1;
  }

  const char *ttf_file_path = argv[1];
  FILE *ttf_file = fopen(ttf_file_path, "rb");
  if (ttf_file == NULL)
  {
    fprintf(stderr, "Failed to open TTF file: %s\n", ttf_file_path);
    return 1;
  }

  TTFTableDirectory *directory = read_ttf_table_directory(ttf_file);
  fclose(ttf_file);
  if (directory == NULL)
  {
    fprintf(stderr, "Failed to read TTF table directory from file: %s\n", ttf_file_path);
    return 1;
  }

  print_ttf_table_directory(directory);
}