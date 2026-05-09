#include "central_directory.h"

void
end_of_central_directory_record_print(EndOfCentralDirectoryRecord *eocd_record)
{
  printf("----- End of Central Directory Record -----\n");
  printf("  Signature:                    0x%08x\n", eocd_record->signature);
  printf("  Disk Number:                  %u\n", eocd_record->disk_number);
  printf("  Central Directory Start Disk: %u\n", eocd_record->central_directory_start_disk);
  printf("  Total Entries on Disk:        %u\n", eocd_record->total_entries_disk);
  printf("  Total Entries:                %u\n", eocd_record->total_entries);
  printf("  Central Directory Size:       %u bytes\n", eocd_record->central_directory_size);
  printf("  Central Directory Offset:     %u bytes\n", eocd_record->central_directory_offset);
  printf("  Comment Length:               %u bytes\n", eocd_record->comment_length);
  if (eocd_record->comment != NULL)
  {
    printf("  Comment: %s\n", eocd_record->comment);
  }
  printf("-------------------------------------------\n");
}

void
central_directory_header_print(CentralDirectoryHeader *cd_header)
{
  printf("  Signature:                    0x%08x\n", cd_header->fixed->signature);
  printf("  Version Made By:              %u\n", cd_header->fixed->version_made_by);
  printf("  Version Needed to Extract:    %u\n", cd_header->fixed->version_needed_to_extract);
  printf("  General Purpose Bit Flag:     0x%04x\n", cd_header->fixed->general_purpose_bit_flag);
  printf("  Compression Method:           %u\n", cd_header->fixed->compression_method);
  printf("  Last Mod File Time:           %u\n", cd_header->fixed->last_mod_file_time);
  printf("  Last Mod File Date:           %u\n", cd_header->fixed->last_mod_file_date);
  printf("  CRC-32:                       0x%08x\n", cd_header->fixed->crc32);
  printf("  Compressed Size:              %u bytes\n", cd_header->fixed->compressed_size);
  printf("  Uncompressed Size:            %u bytes\n", cd_header->fixed->uncompressed_size);
  printf("  File Name Length:             %u bytes\n", cd_header->fixed->file_name_length);
  if (cd_header->file_name != NULL)
  {
    printf("  File Name: %s\n", cd_header->file_name);
  }
  printf("  Extra Field Length:           %u bytes\n", cd_header->fixed->extra_field_length);
  printf("  File Comment Length:          %u bytes\n", cd_header->fixed->file_comment_length);
  if (cd_header->file_comment != NULL)
  {
    printf("  File Comment: %s\n", cd_header->file_comment);
  }
  printf("---------------------------------------------\n");
}

void
central_directory_header_print_all(CentralDirectoryHeader **cd_header, uint16_t num_headers)
{
  printf("------------- Central Directory Headers -------------\n");
  for (uint16_t i = 0; i < num_headers; i++)
  {
    central_directory_header_print(cd_header[i]);
  }
}