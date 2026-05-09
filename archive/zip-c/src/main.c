#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "central_directory.h"

void
usage(char *program_invocation_short_name)
{
  fprintf(stderr, "Usage: %s [<options>] <zipfile>\n", program_invocation_short_name);
  fprintf(stderr, "Options:\n");
  fprintf(stderr, "  -h, --help    Show this help message and exit.\n");
  fprintf(stderr, "  -v, --version Show version information and exit.\n");
  fprintf(stderr, "  -p, --print   Print the contents of the central directory.\n");
}

char *version = "1.0.0";

int
main(int argc, char **argv)
{
  if (argc < 2)
  {
    usage(argv[0]);
    return 1;
  }

  int is_print = 0;
  for (int i = 1; i < argc - 1; i++)
  {
    if (strcmp(argv[i], "-p") == 0 || strcmp(argv[i], "--print") == 0)
    {
      is_print = 1;
    }
    else if (strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "--help") == 0)
    {
      usage(argv[0]);
      return 0;
    }
    else if (strcmp(argv[i], "-v") == 0 || strcmp(argv[i], "--version") == 0)
    {
      fprintf(stdout, "zip-c version %s\n", version);
      return 0;
    }
    else
    {
      fprintf(stderr, "Unknown option: %s\n", argv[i]);
      usage(argv[0]);
      return 1;
    }
  }

  FILE *file = fopen(argv[argc - 1], "rb");
  if (file == NULL)
  {
    fprintf(stderr, "Failed to open file: %s\n", argv[argc - 1]);
    return 1;
  }

  EndOfCentralDirectoryRecord eocd_record;
  Error error = end_of_central_directory_record_find(file, &eocd_record);
  if (error != ERROR_NONE)
  {
    fprintf(stderr, "Failed to find EOCD record.\n");
    fclose(file);
    return 1;
  }

  if (is_print)
  {
    end_of_central_directory_record_print(&eocd_record);
  }

  CentralDirectoryHeader *headers = NULL;
  error = central_directory_header_read_all(file, &headers, &eocd_record);
  if (error != ERROR_NONE)
  {
    fprintf(stderr, "Failed to read central directory headers.\n");
    end_of_central_directory_record_free(&eocd_record);
    fclose(file);
    return 1;
  }

  if (is_print)
  {
    central_directory_header_print_all(&headers);
  }

  fclose(file);
  end_of_central_directory_record_free(&eocd_record);
  central_directory_header_free(headers);
  return 0;
}