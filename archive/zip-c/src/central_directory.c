#include "central_directory.h"

Error find_end_of_central_directory_record(FILE *file, EndOfCentralDirectoryRecord *eocd_record)
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

  eocd_record = malloc(sizeof(EndOfCentralDirectoryRecord));
  if (eocd_record == NULL)
  {
    fprintf(stderr, "Failed to allocate memory for EOCD record.\n");
    free(buf);
    return ERROR_MALLOC_FAILED;
  }

  for (long i = size - END_OF_CENTRAL_DIRECTORY_RECORD_SIZE_WITHOUT_COMMENT; i >= 0; i--)
  {
    if (memcmp(buf + i, END_OF_CENTRAL_DIRECTORY_RECORD_SIGNATURE, 4) == 0)
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

Error read_central_directory_header(FILE *file, CentralDirectoryHeader *cd_header)
{
  cd_header = malloc(sizeof(CentralDirectoryHeader));
  if (cd_header == NULL)
  {
    fprintf(stderr, "Failed to allocate memory for central directory header.\n");
    return ERROR_MALLOC_FAILED;
  }

  size_t read_size = fread(cd_header, sizeof(CentralDirectoryHeader) - sizeof(char *) * 3, 1, file);
  if (read_size != 1)
  {
    fprintf(stderr, "Failed to read central directory header.\n");
    return ERROR_FILE_IO_FAILED;
  }

  if (cd_header->signature != CENTRAL_DIRECTORY_HEADER_SIGNATURE)
  {
    fprintf(stderr, "Invalid central directory header signature.\n");
    return ERROR_FILE_IO_FAILED;
  }

  if (cd_header->file_name_length == 0)
  {
    cd_header->file_name = NULL;
  }
  else
  {
    cd_header->file_name = (char *)malloc(cd_header->file_name_length + 1);
    if (cd_header->file_name == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for file name.\n");
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(cd_header->file_name, sizeof(char), cd_header->file_name_length, file);
    if (read_size != cd_header->file_name_length)
    {
      fprintf(stderr, "Failed to read file name.\n");
      free(cd_header->file_name);
      return ERROR_FILE_IO_FAILED;
    }
    cd_header->file_name[cd_header->file_name_length] = '\0';
  }

  if (cd_header->extra_field_length == 0)
  {
    cd_header->extra_field = NULL;
  }
  else
  {
    cd_header->extra_field = (char *)malloc(cd_header->extra_field_length);
    if (cd_header->extra_field == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for extra field.\n");
      free(cd_header->file_name);
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(cd_header->extra_field, sizeof(char), cd_header->extra_field_length, file);
    if (read_size != cd_header->extra_field_length)
    {
      fprintf(stderr, "Failed to read extra field.\n");
      free(cd_header->file_name);
      free(cd_header->extra_field);
      return ERROR_FILE_IO_FAILED;
    }
  }

  if (cd_header->file_comment_length == 0)
  {
    cd_header->file_comment = NULL;
  }
  else
  {
    cd_header->file_comment = (char *)malloc(cd_header->file_comment_length + 1);
    if (cd_header->file_comment == NULL)
    {
      fprintf(stderr, "Failed to allocate memory for file comment.\n");
      free(cd_header->file_name);
      free(cd_header->extra_field);
      return ERROR_MALLOC_FAILED;
    }

    read_size = fread(cd_header->file_comment, sizeof(char), cd_header->file_comment_length, file);
    if (read_size != cd_header->file_comment_length)
    {
      fprintf(stderr, "Failed to read file comment.\n");
      free(cd_header->file_name);
      free(cd_header->extra_field);
      free(cd_header->file_comment);
      return ERROR_FILE_IO_FAILED;
    }
    cd_header->file_comment[cd_header->file_comment_length] = '\0';
  }

  return ERROR_NONE;
}