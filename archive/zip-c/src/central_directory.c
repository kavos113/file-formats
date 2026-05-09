#include "central_directory.h"

#include <stdlib.h>
#include <string.h>

Error end_of_central_directory_record_find(FILE *file, EndOfCentralDirectoryRecord *eocd_record)
{
  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);

  long size = END_OF_CENTRAL_DIRECTORY_RECORD_MAX_SIZE;
  if (file_size < size)
  {
    size = file_size;
    fseek(file, 0, SEEK_SET);
  }
  else
  {
    fseek(file, -size, SEEK_END);
  }

  char *buf = (char *)malloc(size);
  if (buf == NULL)
  {
    fprintf(stderr, "Failed to allocate memory for EOCD search buffer.\n");
    return ERROR_MALLOC_FAILED;
  }

  size_t read_size = fread(buf, sizeof(char), size, file);
  if (read_size != size)
  {
    fprintf(stderr, "Failed to read EOCD search buffer.\n");
    free(buf);
    return ERROR_FILE_IO_FAILED;
  }

  for (long i = size - END_OF_CENTRAL_DIRECTORY_RECORD_SIZE_WITHOUT_COMMENT; i >= 0; i--)
  {
    if ((*(uint32_t *)(buf + i)) == END_OF_CENTRAL_DIRECTORY_RECORD_SIGNATURE)
    {
      memcpy(eocd_record, buf + i, sizeof(EndOfCentralDirectoryRecord) - sizeof(char *));
      if (eocd_record->comment_length > 0)
      {
        eocd_record->comment = (char *)malloc(eocd_record->comment_length + 1);
        if (eocd_record->comment == NULL)
        {
          fprintf(stderr, "Failed to allocate memory for EOCD comment.\n");
          free(buf);
          free(eocd_record);
          return ERROR_MALLOC_FAILED;
        }
        memcpy(eocd_record->comment, buf + i + sizeof(EndOfCentralDirectoryRecord) - sizeof(char *), eocd_record->comment_length);
        eocd_record->comment[eocd_record->comment_length] = '\0';
      }
      else
      {
        eocd_record->comment = NULL;
      }

      return ERROR_NONE;
    }
  }

  fprintf(stderr, "Failed to find EOCD record.\n");
  free(buf);
  return ERROR_FILE_IO_FAILED;
}

void end_of_central_directory_record_free(EndOfCentralDirectoryRecord *eocd_record)
{
  if (eocd_record->comment != NULL)
  {
    free(eocd_record->comment);
    eocd_record->comment = NULL;
  }
}

Error read_central_directory_header(FILE *file, CentralDirectoryHeader **cd_header)
{
  CentralDirectoryHeader *header = malloc(sizeof(CentralDirectoryHeader));
  if (header == NULL)
  {
    fprintf(stderr, "Failed to allocate memory for central directory header.\n");
    return ERROR_MALLOC_FAILED;
  }

  size_t read_size = fread(header, sizeof(CentralDirectoryHeader) - sizeof(char *) * 3, 1, file);
  if (read_size != 1)
  {
    fprintf(stderr, "Failed to read central directory header.\n");
    return ERROR_FILE_IO_FAILED;
  }

  if (header->signature != CENTRAL_DIRECTORY_HEADER_SIGNATURE)
  {
    fprintf(stderr, "Invalid central directory header signature.\n");
    return ERROR_FILE_IO_FAILED;
  }

  if (header->file_name_length == 0)
  {
    header->file_name = NULL;
  }
  else
  {
    header->file_name = (char *)malloc(header->file_name_length + 1);
    if (header->file_name == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for file name.\n");
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(header->file_name, sizeof(char), header->file_name_length, file);
    if (read_size != header->file_name_length)
    {
      fprintf(stderr, "Failed to read file name.\n");
      free(header->file_name);
      return ERROR_FILE_IO_FAILED;
    }
    header->file_name[header->file_name_length] = '\0';
  }

  if (header->extra_field_length == 0)
  {
    header->extra_field = NULL;
  }
  else
  {
    header->extra_field = (char *)malloc(header->extra_field_length);
    if (header->extra_field == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for extra field.\n");
      free(header->file_name);
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(header->extra_field, sizeof(char), header->extra_field_length, file);
    if (read_size != header->extra_field_length)
    {
      fprintf(stderr, "Failed to read extra field.\n");
      free(header->file_name);
      free(header->extra_field);
      return ERROR_FILE_IO_FAILED;
    }
  }

  if (header->file_comment_length == 0)
  {
    header->file_comment = NULL;
  }
  else
  {
    header->file_comment = (char *)malloc(header->file_comment_length + 1);
    if (header->file_comment == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for file comment.\n");
      free(header->file_name);
      free(header->extra_field);
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(header->file_comment, sizeof(char), header->file_comment_length, file);
    if (read_size != header->file_comment_length)
    {
      fprintf(stderr, "Failed to read file comment.\n");
      free(header->file_name);
      free(header->extra_field);
      free(header->file_comment);
      return ERROR_FILE_IO_FAILED;
    }
    header->file_comment[header->file_comment_length] = '\0';
  }

  *cd_header = header;

  return ERROR_NONE;
}

Error central_directory_header_read_all(FILE *file, CentralDirectoryHeader **cd_header, EndOfCentralDirectoryRecord *eocd_record)
{
}