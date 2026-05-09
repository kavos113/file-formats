#include "central_directory.h"

#include <stdlib.h>
#include <string.h>

Error
end_of_central_directory_record_find(FILE *file, EndOfCentralDirectoryRecord *eocd_record)
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

void
end_of_central_directory_record_free(EndOfCentralDirectoryRecord *eocd_record)
{
  if (eocd_record->comment != NULL)
  {
    free(eocd_record->comment);
    eocd_record->comment = NULL;
  }
}

Error
read_central_directory_header(FILE *file, CentralDirectoryHeader **cd_header, long *size)
{
  CentralDirectoryHeader *header = malloc(sizeof(CentralDirectoryHeader));
  if (header == NULL)
  {
    fprintf(stderr, "Failed to allocate memory for central directory header.\n");
    return ERROR_MALLOC_FAILED;
  }

  CentralDirectoryHeaderFixed *fixed = malloc(sizeof(CentralDirectoryHeaderFixed));
  if (fixed == NULL)
  {
    fprintf(stderr, "Failed to allocate memory for central directory header fixed part.\n");
    free(header);
    return ERROR_MALLOC_FAILED;
  }

  size_t read_size = fread(fixed, sizeof(CentralDirectoryHeaderFixed), 1, file);
  if (read_size != 1)
  {
    fprintf(stderr, "Failed to read central directory header.\n");
    free(fixed);
    free(header);
    return ERROR_FILE_IO_FAILED;
  }
  *size = sizeof(CentralDirectoryHeaderFixed);

  if (fixed->signature != CENTRAL_DIRECTORY_HEADER_SIGNATURE)
  {
    fprintf(stderr, "Invalid central directory header signature: expected 0x%08x, got 0x%08x.\n", CENTRAL_DIRECTORY_HEADER_SIGNATURE, fixed->signature);
    return ERROR_FILE_IO_FAILED;
  }

  if (fixed->file_name_length == 0)
  {
    header->file_name = NULL;
  }
  else
  {
    header->file_name = (char *)malloc(fixed->file_name_length + 1);
    if (header->file_name == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for file name.\n");
      free(fixed);
      free(header);
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(header->file_name, sizeof(char), fixed->file_name_length, file);
    if (read_size != fixed->file_name_length)
    {
      fprintf(stderr, "Failed to read file name.\n");
      free(header->file_name);
      free(fixed);
      free(header);
      return ERROR_FILE_IO_FAILED;
    }
    header->file_name[fixed->file_name_length] = '\0';

    *size += fixed->file_name_length;
  }

  if (fixed->extra_field_length == 0)
  {
    header->extra_field = NULL;
  }
  else
  {
    header->extra_field = (char *)malloc(fixed->extra_field_length);
    if (header->extra_field == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for extra field.\n");
      free(header->file_name);
      free(fixed);
      free(header);
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(header->extra_field, sizeof(char), fixed->extra_field_length, file);
    if (read_size != fixed->extra_field_length)
    {
      fprintf(stderr, "Failed to read extra field.\n");
      free(header->file_name);
      free(header->extra_field);
      free(fixed);
      free(header);
      return ERROR_FILE_IO_FAILED;
    }

    *size += fixed->extra_field_length;
  }

  if (fixed->file_comment_length == 0)
  {
    header->file_comment = NULL;
  }
  else
  {
    header->file_comment = (char *)malloc(fixed->file_comment_length + 1);
    if (header->file_comment == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for file comment.\n");
      free(header->file_name);
      free(header->extra_field);
      free(fixed);
      free(header);
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(header->file_comment, sizeof(char), fixed->file_comment_length, file);
    if (read_size != fixed->file_comment_length)
    {
      fprintf(stderr, "Failed to read file comment.\n");
      free(header->file_name);
      free(header->extra_field);
      free(header->file_comment);
      free(fixed);
      free(header);
      return ERROR_FILE_IO_FAILED;
    }
    header->file_comment[fixed->file_comment_length] = '\0';

    *size += fixed->file_comment_length;
  }

  *cd_header = header;

  return ERROR_NONE;
}

Error
central_directory_header_read_all(FILE *file, CentralDirectoryHeader **cd_header, EndOfCentralDirectoryRecord *eocd_record)
{
  fseek(file, eocd_record->central_directory_offset, SEEK_SET);

  long total_size = 0;
  for (uint16_t i = 0; i < eocd_record->total_entries; i++)
  {
    long size = 0;
    CentralDirectoryHeader *header = NULL;
    Error error = read_central_directory_header(file, &header, &size);
    if (error != ERROR_NONE)
    {
      fprintf(stderr, "Failed to read central directory header %u.\n", i);
      return error;
    }

    cd_header[i] = header;
    total_size += size;

    printf("Read central directory header %u: size = %ld bytes\n", i, size);
  }

  if (total_size != eocd_record->central_directory_size)
  {
    fprintf(stderr, "Central directory size mismatch: expected %u bytes, read %ld bytes.\n", eocd_record->central_directory_size, total_size);
    return ERROR_FILE_IO_FAILED;
  }

  return ERROR_NONE;
}

void
central_directory_header_free_all(CentralDirectoryHeader **cd_header, uint16_t num_headers)
{
  for (uint16_t i = 0; i < num_headers; i++)
  {
    if (cd_header[i] != NULL)
    {
      free(cd_header[i]->file_name);
      free(cd_header[i]->extra_field);
      free(cd_header[i]->file_comment);
      free(cd_header[i]);
      cd_header[i] = NULL;
    }
  }
}