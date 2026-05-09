#include "central_directory.h"

void end_of_central_directory_record_print(EndOfCentralDirectoryRecord *eocd_record)
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