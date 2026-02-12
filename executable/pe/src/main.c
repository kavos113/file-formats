#include <stdio.h>

#include "header.h"
#include "pe.h"
#include "print.h"

void usage()
{
  fprintf(stderr, "usage: readpe <pe file>\n");
}

int main(int argc, char **argv)
{
  if (argc != 2)
  {
    usage();
    return 1;
  }

  const char *pe_file_path = argv[1];
  FILE *pe_file = fopen(pe_file_path, "rb");
  if (pe_file == NULL)
  {
    fprintf(stderr, "Failed to open PE file: %s\n", pe_file_path);
    return 1;
  }

  PEHeader *pe_header = read_pe_header(pe_file);
  fclose(pe_file);
  if (pe_header == NULL)
  {
    fprintf(stderr, "Failed to read PE header from file: %s\n", pe_file_path);
    return 1;
  }

  print_pe_header(pe_header);
  return 0;
}