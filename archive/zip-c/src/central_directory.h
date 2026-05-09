#ifndef CENTRAL_DIRECTORY_H
#define CENTRAL_DIRECTORY_H

#include <stdint.h>
#include <stdio.h>

#include "error.h"

typedef struct EndOfCentralDirectoryRecord
{
  uint32_t signature;
  uint16_t disk_number;
  uint16_t central_directory_start_disk;
  uint16_t total_entries_disk;
  uint16_t total_entries;
  uint32_t central_directory_size;
  uint32_t central_directory_offset;
  uint16_t comment_length;
  char *comment;
} EndOfCentralDirectoryRecord;

#define END_OF_CENTRAL_DIRECTORY_RECORD_SIGNATURE 0x06054b50
#define END_OF_CENTRAL_DIRECTORY_RECORD_SIZE_WITHOUT_COMMENT 22
#define END_OF_CENTRAL_DIRECTORY_RECORD_MAX_COMMENT_SIZE 65535
#define END_OF_CENTRAL_DIRECTORY_RECORD_MAX_SIZE (END_OF_CENTRAL_DIRECTORY_RECORD_SIZE_WITHOUT_COMMENT + END_OF_CENTRAL_DIRECTORY_RECORD_MAX_COMMENT_SIZE)

typedef struct CentralDirectoryHeader
{
  uint32_t signature;
  uint16_t version_made_by;
  uint16_t version_needed_to_extract;
  uint16_t general_purpose_bit_flag;
  uint16_t compression_method;
  uint16_t last_mod_file_time;
  uint16_t last_mod_file_date;
  uint32_t crc32;
  uint32_t compressed_size;
  uint32_t uncompressed_size;
  uint16_t file_name_length;
  uint16_t extra_field_length;
  uint16_t file_comment_length;
  uint16_t disk_number_start;
  uint16_t internal_file_attributes;
  uint32_t external_file_attributes;
  uint32_t relative_offset_local_header;
  char *file_name;
  char *extra_field;
  char *file_comment;
} CentralDirectoryHeader;

#define CENTRAL_DIRECTORY_HEADER_SIGNATURE 0x02014b50

Error end_of_central_directory_record_find(FILE *file, EndOfCentralDirectoryRecord *eocd_record);
void end_of_central_directory_record_free(EndOfCentralDirectoryRecord *eocd_record);
void end_of_central_directory_record_print(EndOfCentralDirectoryRecord *eocd_record);
Error central_directory_header_read_all(FILE *file, CentralDirectoryHeader **cd_header, EndOfCentralDirectoryRecord *eocd_record);
Error central_directory_header_free(CentralDirectoryHeader *cd_header);
Error central_directory_header_print(CentralDirectoryHeader *cd_header);

#define CENTRAL_DIRECTORY_VERSION_MADE_BY_MSDOS 0
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_AMIGA 1
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_OPENVMS 2
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_UNIX 3
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_VM_CMS 4
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_ATARI_ST 5
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_OS2_HPFS 6
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_MACOS 7
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_Z_SYSTEM 8
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_CPM 9
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_WINDOWS_NTFS 10
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_MVS 11
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_VSE 12
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_ACORN_RISC 13
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_VFAT 14
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_ALTERNATE_MVS 15
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_BEOS 16
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_TANDEM 17
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_OS400 18
#define CENTRAL_DIRECTORY_VERSION_MADE_BY_OSX 19

#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_STORE 0
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_SHRUNK 1
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_REDUCED_1 2
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_REDUCED_2 3
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_REDUCED_3 4
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_REDUCED_4 5
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_IMPLODED 6
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_DEFLATED 8
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_DEFLATED64 9
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_PKWARE_IMPLODED 10
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_BZIP2 12
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_LZMA 14
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_IBM_CMPSC 16
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_IBM_TERSE 18
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_IBM_LZ77 19
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_ZSTD 93
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_MP3 94
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_XZ 95
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_JPEG 96
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_WAVPACK 97
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_PPMD 98
#define CENTRAL_DIRECTORY_COMPRESSION_METHOD_AEX_ENCRYPTION_MARKER 99

#endif // CENTRAL_DIRECTORY_H