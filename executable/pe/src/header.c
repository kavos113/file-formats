#include "header.h"
#include "error.h"

#include <stdint.h>
#include <stdlib.h>

PEError seek_pe_header(FILE *file);
PEError read_signature(FILE *file, char *signature);
// expected file pointer to be at the start of COFF header
PEError read_coff_header(FILE *file, COFFHeader *coff_header);
PEError read_optional_header(FILE *file, OptionalHeader *optional_header);
PEError read_section_table(FILE *file, SectionHeader **section_table, uint16_t number_of_sections);

PEHeader *read_pe_header(FILE *file)
{
  PEHeader *pe_header = malloc(sizeof(PEHeader));
  if (pe_header == NULL)
  {
    fprintf(stderr, "Memory allocation failed.\n");
    return NULL;
  }

  PEError err = seek_pe_header(file);
  if (err != ERROR_NONE)
  {
    free(pe_header);
    return NULL;
  }

  err = read_signature(file, pe_header->signature);
  if (err != ERROR_NONE)
  {
    free(pe_header);
    return NULL;
  }

  err = read_coff_header(file, &pe_header->coff_header);
  if (err != ERROR_NONE)
  {
    free(pe_header);
    return NULL;
  }

  err = read_optional_header(file, &pe_header->optional_header);
  if (err != ERROR_NONE)
  {
    free(pe_header);
    return NULL;
  }

  err = read_section_table(
      file,
      &pe_header->section_table,
      pe_header->coff_header.number_of_sections);
  if (err != ERROR_NONE)
  {
    free(pe_header);
    return NULL;
  }

  return pe_header;
}

PEError seek_pe_header(FILE *file)
{
  if (file == NULL)
  {
    fprintf(stderr, "Invalid file pointer.\n");
    return ERROR_INVALID_FILE_POINTER;
  }

  if (fseek(file, DOS_STUB_PE_HEADER_ADDRESS, SEEK_SET) != 0)
  {
    fprintf(stderr, "Failed to seek to PE header address.\n");
    return ERROR_FILE_SEEK_FAILED;
  }

  uint32_t pe_header_offset;
  size_t read_size = fread(&pe_header_offset, sizeof(uint32_t), 1, file);
  if (read_size != 1)
  {
    fprintf(stderr, "Failed to read PE header offset.\n");
    return ERROR_FILE_READ_FAILED;
  }

  if (fseek(file, pe_header_offset, SEEK_SET) != 0)
  {
    fprintf(stderr, "Failed to seek to PE header.\n");
    return ERROR_FILE_SEEK_FAILED;
  }

  return ERROR_NONE;
}

PEError read_signature(FILE *file, char *signature)
{
  if (file == NULL)
  {
    fprintf(stderr, "Invalid file pointer.\n");
    return ERROR_INVALID_FILE_POINTER;
  }

  size_t read_size = fread(signature, sizeof(char), 4, file);
  if (read_size != 4)
  {
    fprintf(stderr, "Failed to read PE signature.\n");
    return ERROR_FILE_READ_FAILED;
  }

  return ERROR_NONE;
}

PEError read_coff_header(FILE *file, COFFHeader *coff_header)
{
  if (file == NULL)
  {
    fprintf(stderr, "Invalid file pointer.\n");
    return ERROR_INVALID_FILE_POINTER;
  }

  size_t read_size = fread(coff_header, sizeof(COFFHeader), 1, file);
  if (read_size != 1)
  {
    fprintf(stderr, "Failed to read COFF header.\n");
    return ERROR_FILE_READ_FAILED;
  }

  return ERROR_NONE;
}

PEError read_optional_header(FILE *file, OptionalHeader *optional_header)
{
  if (file == NULL)
  {
    fprintf(stderr, "Invalid file pointer.\n");
    return ERROR_INVALID_FILE_POINTER;
  }

  uint16_t magic;
  size_t read_size = fread(&magic, sizeof(uint16_t), 1, file);
  if (read_size != 1)
  {
    fprintf(stderr, "Failed to read Optional Header magic.\n");
    return ERROR_FILE_READ_FAILED;
  }

  switch (magic)
  {
  case PE_OPTIONAL_HEADER_MAGIC_PE32:
    optional_header->magic = magic;
    // Move file pointer back to read the full Optional Header
    if (fseek(file, -(long)(sizeof(uint16_t)), SEEK_CUR) != 0)
    {
      fprintf(stderr, "Failed to seek back to Optional Header start.\n");
      return ERROR_FILE_SEEK_FAILED;
    }

    read_size = fread(&optional_header->pe32, sizeof(OptionalHeaderPE32), 1, file);
    if (read_size != 1)
    {
      fprintf(stderr, "Failed to read PE32 Optional Header.\n");
      return ERROR_FILE_READ_FAILED;
    }
    break;

  case PE_OPTIONAL_HEADER_MAGIC_PE32PLUS:
    optional_header->magic = magic;
    // Move file pointer back to read the full Optional Header
    if (fseek(file, -(long)(sizeof(uint16_t)), SEEK_CUR) != 0)
    {
      fprintf(stderr, "Failed to seek back to Optional Header start.\n");
      return ERROR_FILE_SEEK_FAILED;
    }

    read_size = fread(&optional_header->pe32_plus, sizeof(OptionalHeaderPE32Plus), 1, file);
    if (read_size != 1)
    {
      fprintf(stderr, "Failed to read PE32+ Optional Header.\n");
      return ERROR_FILE_READ_FAILED;
    }
    break;
  default:
    fprintf(stderr, "Unknown Optional Header magic: 0x%04x\n", magic);
    return ERROR_UNKNOWN_OPTIONAL_HEADER_MAGIC;
  }

  return ERROR_NONE;
}

PEError read_section_table(FILE *file, SectionHeader **section_table, uint16_t number_of_sections)
{
  if (file == NULL)
  {
    fprintf(stderr, "Invalid file pointer.\n");
    return ERROR_INVALID_FILE_POINTER;
  }

  *section_table = malloc(sizeof(SectionHeader) * number_of_sections);
  if (*section_table == NULL)
  {
    fprintf(stderr, "Memory allocation for section table failed.\n");
    return ERROR_FILE_READ_FAILED;
  }

  size_t read_size = fread(*section_table, sizeof(SectionHeader), number_of_sections, file);
  if (read_size != number_of_sections)
  {
    fprintf(stderr, "Failed to read section table.\n");
    free(*section_table);
    return ERROR_FILE_READ_FAILED;
  }

  return ERROR_NONE;
}