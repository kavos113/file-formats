#ifndef PE_H
#define PE_H

#include <stdint.h>

#define DOS_STUB_PE_HEADER_ADDRESS 0x3C

typedef struct _COFFHeader
{
  uint16_t machine;
  uint16_t number_of_sections;
  uint32_t time_date_stamp;
  uint32_t pointer_to_symbol_table;
  uint32_t number_of_symbols;
  uint16_t size_of_optional_header;
  uint16_t characteristics;
} COFFHeader;

// ----- machine -----
#define IMAGE_FILE_MACHINE_UNKNOWN     0x0
#define IMAGE_FILE_MACHINE_ALPHA       0x184
#define IMAGE_FILE_MACHINE_ALPHA64     0x284
#define IMAGE_FILE_MACHINE_AM33        0x1d3
#define IMAGE_FILE_MACHINE_AMD64       0x8664
#define IMAGE_FILE_MACHINE_ARM         0x1c0
#define IMAGE_FILE_MACHINE_ARM64       0xaa64
#define IMAGE_FILE_MACHINE_ARM64EC     0xa641
#define IMAGE_FILE_MACHINE_ARM64X      0xa64e
#define IMAGE_FILE_MACHINE_ARMNT       0x1c4
#define IMAGE_FILE_MACHINE_AXP64       0x284
#define IMAGE_FILE_MACHINE_EBC         0xebc
#define IMAGE_FILE_MACHINE_I386        0x14c
#define IMAGE_FILE_MACHINE_IA64        0x200
#define IMAGE_FILE_MACHINE_LOONGARCH32 0x6232
#define IMAGE_FILE_MACHINE_LOONGARCH64 0x6264
#define IMAGE_FILE_MACHINE_M32R        0x9041
#define IMAGE_FILE_MACHINE_MIPS16      0x266
#define IMAGE_FILE_MACHINE_MIPSFPU     0x366
#define IMAGE_FILE_MACHINE_MIPSFPU16   0x466
#define IMAGE_FILE_MACHINE_POWERPC     0x1f0
#define IMAGE_FILE_MACHINE_POWERPCFP   0x1f1
#define IMAGE_FILE_MACHINE_R3000BE     0x160
#define IMAGE_FILE_MACHINE_R3000       0x162
#define IMAGE_FILE_MACHINE_R4000       0x166
#define IMAGE_FILE_MACHINE_R10000      0x168
#define IMAGE_FILE_MACHINE_RISCV32     0x5032
#define IMAGE_FILE_MACHINE_RISCV64     0x5064
#define IMAGE_FILE_MACHINE_RISCV128    0x5128
#define IMAGE_FILE_MACHINE_SH3         0x1a2
#define IMAGE_FILE_MACHINE_SH3DSP      0x1a3
#define IMAGE_FILE_MACHINE_SH4         0x1a6
#define IMAGE_FILE_MACHINE_SH5         0x1a8
#define IMAGE_FILE_MACHINE_THUMB       0x1c2
#define IMAGE_FILE_MACHINE_WCEMIPSV2   0x169

// ----- characteristics -----
#define IMAGE_FILE_RELOCS_STRIPPED         0x0001
#define IMAGE_FILE_EXECUTABLE_IMAGE        0x0002
#define IMAGE_FILE_LINE_NUMS_STRIPPED      0x0004
#define IMAGE_FILE_LOCAL_SYMS_STRIPPED     0x0008
#define IMAGE_FILE_AGGRESSIVE_WS_TRIM      0x0010
#define IMAGE_FILE_LARGE_ADDRESS_AWARE     0x0020
#define IMAGE_FILE_BYTES_REVERSED_LO       0x0080
#define IMAGE_FILE_32BIT_MACHINE           0x0100
#define IMAGE_FILE_DEBUG_STRIPPED          0x0200
#define IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP 0x0400
#define IMAGE_FILE_NET_RUN_FROM_SWAP       0x0800
#define IMAGE_FILE_SYSTEM                  0x1000
#define IMAGE_FILE_DLL                     0x2000
#define IMAGE_FILE_UP_SYSTEM_ONLY          0x4000
#define IMAGE_FILE_BYTES_REVERSED_HI       0x8000

#define PE_OPTIONAL_HEADER_MAGIC_PE32     0x10b
#define PE_OPTIONAL_HEADER_MAGIC_PE32PLUS 0x20b

typedef struct _OptionalHeaderStandardPE32
{
  uint16_t magic;
  uint8_t major_linker_version;
  uint8_t minor_linker_version;
  uint32_t size_of_code;
  uint32_t size_of_initialized_data;
  uint32_t size_of_bss_data;
  uint32_t address_of_entry_point;
  uint32_t base_of_code;
  uint32_t base_of_data;
} OptionalHeaderStandardPE32;

typedef struct _OptionalHeaderStandardPE32Plus
{
  uint16_t magic;
  uint8_t major_linker_version;
  uint8_t minor_linker_version;
  uint32_t size_of_code;
  uint32_t size_of_initialized_data;
  uint32_t size_of_bss_data;
  uint32_t address_of_entry_point;
  uint32_t base_of_code;
} OptionalHeaderStandardPE32Plus;

typedef struct _OptionalHeaderWindowsFieldPE32
{
  uint32_t image_base;
  uint32_t section_alignment;
  uint32_t file_alignment;
  uint16_t major_os_version;
  uint16_t minor_os_version;
  uint16_t major_image_version;
  uint16_t minor_image_version;
  uint16_t major_subsystem_version;
  uint16_t minor_subsystem_version;
  uint32_t win32_version_value;
  uint32_t size_of_image;
  uint32_t size_of_headers;
  uint32_t check_sum;
  uint16_t subsystem;
  uint16_t dll_characteristics;
  uint32_t size_of_stack_reserve;
  uint32_t size_of_stack_commit;
  uint32_t size_of_heap_reserve;
  uint32_t size_of_heap_commit;
  uint32_t loader_flags;
  uint32_t number_of_rva_and_sizes;
} OptionalHeaderWindowsFieldPE32;

typedef struct _OptionalHeaderWindowsFieldPE32Plus
{
  uint64_t image_base;
  uint32_t section_alignment;
  uint32_t file_alignment;
  uint16_t major_os_version;
  uint16_t minor_os_version;
  uint16_t major_image_version;
  uint16_t minor_image_version;
  uint16_t major_subsystem_version;
  uint16_t minor_subsystem_version;
  uint32_t win32_version_value;
  uint32_t size_of_image;
  uint32_t size_of_headers;
  uint32_t check_sum;
  uint16_t subsystem;
  uint16_t dll_characteristics;
  uint64_t size_of_stack_reserve;
  uint64_t size_of_stack_commit;
  uint64_t size_of_heap_reserve;
  uint64_t size_of_heap_commit;
  uint32_t loader_flags;
  uint32_t number_of_rva_and_sizes;
} OptionalHeaderWindowsFieldPE32Plus;

typedef struct _OptionalHeaderDataDirectoryEntry
{
  uint32_t virtual_address;
  uint32_t size;
} OptionalHeaderDataDirectoryEntry;

typedef struct _OptionalHeaderDataDirectory
{
  OptionalHeaderDataDirectoryEntry export_table;
  OptionalHeaderDataDirectoryEntry import_table;
  OptionalHeaderDataDirectoryEntry resource_table;
  OptionalHeaderDataDirectoryEntry exception_table;
  OptionalHeaderDataDirectoryEntry certificate_table;
  OptionalHeaderDataDirectoryEntry base_relocation_table;
  OptionalHeaderDataDirectoryEntry debug;
  OptionalHeaderDataDirectoryEntry architecture;
  OptionalHeaderDataDirectoryEntry global_ptr;
  OptionalHeaderDataDirectoryEntry tls_table;
  OptionalHeaderDataDirectoryEntry load_config_table;
  OptionalHeaderDataDirectoryEntry bound_import;
  OptionalHeaderDataDirectoryEntry import_address_table;
  OptionalHeaderDataDirectoryEntry delay_import_descriptor;
  OptionalHeaderDataDirectoryEntry clr_runtime_header;
  OptionalHeaderDataDirectoryEntry reserved;
} OptionalHeaderDataDirectory;

typedef struct _OptionalHeaderPE32
{
  OptionalHeaderStandardPE32 standard_fields;
  OptionalHeaderWindowsFieldPE32 windows_fields;
  OptionalHeaderDataDirectory data_directories;
} OptionalHeaderPE32;

typedef struct _OptionalHeaderPE32Plus
{
  OptionalHeaderStandardPE32Plus standard_fields;
  OptionalHeaderWindowsFieldPE32Plus windows_fields;
  OptionalHeaderDataDirectory data_directories;
} OptionalHeaderPE32Plus;

typedef union _OptionalHeader
{
  uint16_t magic;
  OptionalHeaderPE32 pe32;
  OptionalHeaderPE32Plus pe32_plus;
} OptionalHeader;

// ----- Windows Subsystem -----
#define IMAGE_SUBSYSTEM_UNKNOWN                  0
#define IMAGE_SUBSYSTEM_NATIVE                   1
#define IMAGE_SUBSYSTEM_WINDOWS_GUI              2
#define IMAGE_SUBSYSTEM_WINDOWS_CUI              3
#define IMAGE_SUBSYSTEM_OS2_CUI                  5
#define IMAGE_SUBSYSTEM_POSIX_CUI                7
#define IMAGE_SUBSYSTEM_NATIVE_WINDOWS           8
#define IMAGE_SUBSYSTEM_WINDOWS_CE_GUI           9
#define IMAGE_SUBSYSTEM_EFI_APPLICATION          10
#define IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER  11
#define IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER       12
#define IMAGE_SUBSYSTEM_EFI_ROM                  13
#define IMAGE_SUBSYSTEM_XBOX                     14
#define IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION 16

// ----- DLL Characteristics -----
#define IMAGE_DLL_CHARACTERISTICS_HIGH_ENTROPY_VA       0x0020
#define IMAGE_DLL_CHARACTERISTICS_DYNAMIC_BASE          0x0040
#define IMAGE_DLL_CHARACTERISTICS_FORCE_INTEGRITY       0x0080
#define IMAGE_DLL_CHARACTERISTICS_NX_COMPAT             0x0100
#define IMAGE_DLL_CHARACTERISTICS_NO_ISOLATION          0x0200
#define IMAGE_DLL_CHARACTERISTICS_NO_SEH                0x0400
#define IMAGE_DLL_CHARACTERISTICS_NO_BIND               0x0800
#define IMAGE_DLL_CHARACTERISTICS_APPCONTAINER          0x1000
#define IMAGE_DLL_CHARACTERISTICS_WDM_DRIVER            0x2000
#define IMAGE_DLL_CHARACTERISTICS_GUARD_CF              0x4000
#define IMAGE_DLL_CHARACTERISTICS_TERMINAL_SERVER_AWARE 0x8000

typedef struct _SectionHeader 
{
  char name[8];
  uint32_t virtual_size;
  uint32_t virtual_address;
  uint32_t size_of_raw_data;
  uint32_t ptr_to_raw_data;
  uint32_t ptr_to_relocations;
  uint32_t ptr_to_linenumbers;
  uint16_t number_of_relocations;
  uint16_t number_of_linenumbers;
  uint32_t characteristics;
} SectionHeader;

// ----- Section Characteristics -----
#define IMAGE_SCN_TYPE_NO_PAD             0x00000008
#define IMAGE_SCN_CNT_CODE                0x00000020
#define IMAGE_SCN_CNT_INITIALIZED_DATA    0x00000040
#define IMAGE_SCN_CNT_UNINITIALIZED_DATA  0x00000080
#define IMAGE_SCN_LNK_OTHER               0x00000100
#define IMAGE_SCN_LNK_INFO                0x00000200
#define IMAGE_SCN_LNK_REMOVE              0x00000800
#define IMAGE_SCN_LNK_COMDAT              0x00001000
#define IMAGE_SCN_GPREL                   0x00008000
#define IMAGE_SCN_MEM_PURGEABLE           0x00020000
#define IMAGE_SCN_MEM_16BIT               0x00020000
#define IMAGE_SCN_MEM_LOCKED              0x00040000
#define IMAGE_SCN_MEM_PRELOAD             0x00080000
#define IMAGE_SCN_ALIGN_1BYTES            0x00100000
#define IMAGE_SCN_ALIGN_2BYTES            0x00200000
#define IMAGE_SCN_ALIGN_4BYTES            0x00300000
#define IMAGE_SCN_ALIGN_8BYTES            0x00400000
#define IMAGE_SCN_ALIGN_16BYTES           0x00500000  
#define IMAGE_SCN_ALIGN_32BYTES           0x00600000
#define IMAGE_SCN_ALIGN_64BYTES           0x00700000
#define IMAGE_SCN_ALIGN_128BYTES          0x00800000
#define IMAGE_SCN_ALIGN_256BYTES          0x00900000
#define IMAGE_SCN_ALIGN_512BYTES          0x00A00000
#define IMAGE_SCN_ALIGN_1024BYTES         0x00B00000
#define IMAGE_SCN_ALIGN_2048BYTES         0x00C00000
#define IMAGE_SCN_ALIGN_4096BYTES         0x00D00000  
#define IMAGE_SCN_ALIGN_8192BYTES         0x00E00000
#define IMAGE_SCN_LNK_NRELOC_OVFL         0x01000000
#define IMAGE_SCN_MEM_DISCARDABLE         0x02000000
#define IMAGE_SCN_MEM_NOT_CACHED          0x04000000
#define IMAGE_SCN_MEM_NOT_PAGED           0x08000000
#define IMAGE_SCN_MEM_SHARED              0x10000000
#define IMAGE_SCN_MEM_EXECUTE             0x20000000
#define IMAGE_SCN_MEM_READ                0x40000000
#define IMAGE_SCN_MEM_WRITE               0x80000000

typedef struct _PEHeader
{
  char signature[4];
  COFFHeader coff_header;
  OptionalHeader optional_header;
  SectionHeader *section_table;
} PEHeader;


#endif // PE_H