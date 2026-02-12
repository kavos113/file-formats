#include "print.h"

#include <stdio.h>
#include <time.h>

void print_signature(char *signature);
void print_coff_header(COFFHeader *coff_header);
void print_optional_header(OptionalHeader *optional_header);
void print_section_table(SectionHeader *section_table, uint16_t number_of_sections);

void print_pe_header(PEHeader *pe_header)
{
  if (pe_header == NULL)
  {
    fprintf(stderr, "Invalid PE header pointer.\n");
    return;
  }

  printf("PE Header:\n");
  print_signature(pe_header->signature);
  print_coff_header(&pe_header->coff_header);
  print_optional_header(&pe_header->optional_header);
  print_section_table(pe_header->section_table, pe_header->coff_header.number_of_sections);
}

void print_pretty_char(char c)
{
  switch (c)
  {
  case 0:
    printf("\\0");
    return;
  case '\n':
    printf("\\n");
    return;
  case '\r':
    printf("\\r");
    return;
  case '\t':
    printf("\\t");
    return;
  case '\\':
    printf("\\\\");
    return;
  case ' ':
    printf(" ");
    return;
  }

  if (c >= 0x20 && c <= 0x7E)
  {
    printf("%c", c);
  }
  else
  {
    printf("\\x%02x", (unsigned char)c);
  }
}

void print_characteristic_flag(const char *description, int indent_size)
{
  for (int i = 0; i < indent_size; i++)
  {
    printf(" ");
  }
  printf("- %s\n", description);
}

void print_version(uint16_t major, uint16_t minor, int indent_size)
{
  for (int i = 0; i < indent_size; i++)
  {
    printf(" ");
  }
  printf("%u.%u\n", major, minor);
}

void print_signature(char *signature)
{
  printf("Signature: ");
  for (int i = 0; i < 4; i++)
  {
    print_pretty_char(signature[i]);
    printf(" ");
  }
  printf("\n");
}

void print_coff_machine(uint16_t machine);
void print_coff_characteristics(uint16_t characteristics, int indent_size);
void print_coff_timestamp(uint32_t time_date_stamp);

void print_coff_header(COFFHeader *coff_header)
{
  if (coff_header == NULL)
  {
    fprintf(stderr, "Invalid COFF header pointer.\n");
    return;
  }

  printf("\n");
  printf("--------- COFF Header ---------\n");
  printf("Machine:                 ");
  print_coff_machine(coff_header->machine);
  printf("Number of Sections:      %u\n", coff_header->number_of_sections);
  printf("Latest Timestamp:        ");
  print_coff_timestamp(coff_header->time_date_stamp);
  printf("Pointer to Symbol Table: %u\n", coff_header->pointer_to_symbol_table);
  printf("Number of Symbols:       %u\n", coff_header->number_of_symbols);
  printf("Size of Optional Header: %u\n", coff_header->size_of_optional_header);
  printf("Characteristics:\n");
  print_coff_characteristics(coff_header->characteristics, 2);
  printf("-------------------------------\n");
}

void print_coff_machine(uint16_t machine)
{
  switch (machine)
  {
  case IMAGE_FILE_MACHINE_UNKNOWN:
    printf("Unknown\n");
    break;
  case IMAGE_FILE_MACHINE_ALPHA:
    printf("Alpha AXP, 32-bit\n");
    break;
  case IMAGE_FILE_MACHINE_ALPHA64:
    printf("Alpha 64, 64-bit\n");
    break;
  case IMAGE_FILE_MACHINE_AM33:
    printf("Matsushita AM33\n");
    break;
  case IMAGE_FILE_MACHINE_AMD64:
    printf("x64\n");
    break;
  case IMAGE_FILE_MACHINE_ARM:
    printf("ARM Little-Endian\n");
    break;
  case IMAGE_FILE_MACHINE_ARM64:
    printf("ARM64 Little-Endian\n");
    break;
  case IMAGE_FILE_MACHINE_ARM64EC:
    printf("interoperable between native ARM64 and emulated x64\n");
    break;
  case IMAGE_FILE_MACHINE_ARM64X:
    printf("ARM64 and ARM64EC");
    break;
  case IMAGE_FILE_MACHINE_ARMNT:
    printf("ARM Thumb-2 Little-Endian\n");
    break;
  case IMAGE_FILE_MACHINE_EBC:
    printf("EFI Byte Code\n");
    break;
  case IMAGE_FILE_MACHINE_I386:
    printf("i386\n");
    break;
  case IMAGE_FILE_MACHINE_IA64:
    printf("Intel Itanium\n");
    break;
  case IMAGE_FILE_MACHINE_LOONGARCH32:
    printf("LoongArch 32-bit\n");
    break;
  case IMAGE_FILE_MACHINE_LOONGARCH64:
    printf("LoongArch 64-bit\n");
    break;
  case IMAGE_FILE_MACHINE_M32R:
    printf("Mitsubishi M32R Little-Endian\n");
    break;
  case IMAGE_FILE_MACHINE_MIPS16:
    printf("MIPS16\n");
    break;
  case IMAGE_FILE_MACHINE_MIPSFPU:
    printf("MIPS with FPU\n");
    break;
  case IMAGE_FILE_MACHINE_MIPSFPU16:
    printf("MIPS16 with FPU\n");
    break;
  case IMAGE_FILE_MACHINE_POWERPC:
    printf("Power PC Little-Endian\n");
    break;
  case IMAGE_FILE_MACHINE_POWERPCFP:
    printf("Power PC with Floating Point Support\n");
    break;
  case IMAGE_FILE_MACHINE_R3000BE:
    printf("MIPS I compatible 32-bit big-endian\n");
    break;
  case IMAGE_FILE_MACHINE_R3000:
    printf("MIPS I compatible 32-bit little-endian\n");
    break;
  case IMAGE_FILE_MACHINE_R4000:
    printf("MIPS III compatible 64-bit little-endian\n");
    break;
  case IMAGE_FILE_MACHINE_R10000:
    printf("MIPS IV compatible 64-bit little-endian\n");
    break;
  case IMAGE_FILE_MACHINE_RISCV32:
    printf("RISC-V 32-bit\n");
    break;
  case IMAGE_FILE_MACHINE_RISCV64:
    printf("RISC-V 64-bit\n");
    break;
  case IMAGE_FILE_MACHINE_RISCV128:
    printf("RISC-V 128-bit\n");
    break;
  case IMAGE_FILE_MACHINE_SH3:
    printf("Hitachi SH3\n");
    break;
  case IMAGE_FILE_MACHINE_SH3DSP:
    printf("Hitachi SH3 DSP\n");
    break;
  case IMAGE_FILE_MACHINE_SH4:
    printf("Hitachi SH4\n");
    break;
  case IMAGE_FILE_MACHINE_SH5:
    printf("Hitachi SH5\n");
    break;
  case IMAGE_FILE_MACHINE_THUMB:
    printf("ARM Thumb-2 Little-Endian\n");
    break;
  case IMAGE_FILE_MACHINE_WCEMIPSV2:
    printf("MIPS V2 Little-Endian WCE v2\n");
    break;
  default:
    printf("Unknown (0x%04x)\n", machine);
    break;
  }
}

void print_coff_characteristics(uint16_t characteristics, int indent_size)
{
  if (characteristics & IMAGE_FILE_RELOCS_STRIPPED)
  {
    print_characteristic_flag(
        "Relocation stripped", indent_size);
  }

  if (characteristics & IMAGE_FILE_EXECUTABLE_IMAGE)
  {
    print_characteristic_flag(
        "Executable file", indent_size);
  }

  if (characteristics & IMAGE_FILE_LINE_NUMS_STRIPPED)
  {
    print_characteristic_flag(
        "Line numbers stripped", indent_size);
  }

  if (characteristics & IMAGE_FILE_LOCAL_SYMS_STRIPPED)
  {
    print_characteristic_flag(
        "Local symbols stripped", indent_size);
  }

  if (characteristics & IMAGE_FILE_AGGRESSIVE_WS_TRIM)
  {
    print_characteristic_flag(
        "Aggressively trim working set", indent_size);
  }

  if (characteristics & IMAGE_FILE_LARGE_ADDRESS_AWARE)
  {
    print_characteristic_flag(
        "App can handle >2GB addresses", indent_size);
  }

  if (characteristics & IMAGE_FILE_BYTES_REVERSED_LO)
  {
    print_characteristic_flag(
        "Little endian", indent_size);
  }

  if (characteristics & IMAGE_FILE_32BIT_MACHINE)
  {
    print_characteristic_flag(
        "32-bit word architecture", indent_size);
  }

  if (characteristics & IMAGE_FILE_DEBUG_STRIPPED)
  {
    print_characteristic_flag(
        "Debug info stripped", indent_size);
  }

  if (characteristics & IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP)
  {
    print_characteristic_flag(
        "If Image is on removable media, copy and run from swap", indent_size);
  }

  if (characteristics & IMAGE_FILE_NET_RUN_FROM_SWAP)
  {
    print_characteristic_flag(
        "If Image is on Network media, copy and run from swap", indent_size);
  }

  if (characteristics & IMAGE_FILE_SYSTEM)
  {
    print_characteristic_flag(
        "System file", indent_size);
  }

  if (characteristics & IMAGE_FILE_DLL)
  {
    print_characteristic_flag(
        "DLL file", indent_size);
  }

  if (characteristics & IMAGE_FILE_UP_SYSTEM_ONLY)
  {
    print_characteristic_flag(
        "Only run on a UniProcessor machine", indent_size);
  }

  if (characteristics & IMAGE_FILE_BYTES_REVERSED_HI)
  {
    print_characteristic_flag(
        "Big endian", indent_size);
  }
}

void print_coff_timestamp(uint32_t time_date_stamp)
{
  time_t raw_time = (time_t)time_date_stamp;

  struct tm *time_info = localtime(&raw_time);
  if (time_info == NULL)
  {
    printf("Invalid timestamp: %u\n", time_date_stamp);
    return;
  }

  char buffer[26];
  if (strftime(buffer, sizeof(buffer), "%Y-%m-%d %H:%M:%S", time_info) == 0)
  {
    printf("Invalid timestamp: %u\n", time_date_stamp);
    return;
  }
  printf("%s\n", buffer);
}

void print_optional_header_standard_pe32(OptionalHeaderStandardPE32 *standard_fields);
void print_optional_header_standard_pe32_plus(OptionalHeaderStandardPE32Plus *standard_fields);
void print_optional_header_windows_field_pe32(OptionalHeaderWindowsFieldPE32 *windows_fieds);
void print_optional_header_windows_field_pe32_plus(OptionalHeaderWindowsFieldPE32Plus *windows_fieds);
void print_optional_header_data_directory(OptionalHeaderDataDirectory *data_directories);

void print_optional_header(OptionalHeader *optional_header)
{
  if (optional_header == NULL)
  {
    fprintf(stderr, "Invalid Optional header pointer.\n");
    return;
  }

  printf("\n");
  printf("----------- Optional Header -----------\n");

  if (optional_header->magic == PE_OPTIONAL_HEADER_MAGIC_PE32)
  {
    printf("Format: PE32\n");
    print_optional_header_standard_pe32(&optional_header->pe32.standard_fields);
    print_optional_header_windows_field_pe32(&optional_header->pe32.windows_fields);
    print_optional_header_data_directory(&optional_header->pe32.data_directories);
  }
  else if (optional_header->magic == PE_OPTIONAL_HEADER_MAGIC_PE32PLUS)
  {
    printf("Format: PE32+\n");
    print_optional_header_standard_pe32_plus(&optional_header->pe32_plus.standard_fields);
    print_optional_header_windows_field_pe32_plus(&optional_header->pe32_plus.windows_fields);
    print_optional_header_data_directory(&optional_header->pe32_plus.data_directories);
  }
  else
  {
    printf("Unknown Optional Header magic: 0x%04x\n", optional_header->magic);
  }

  printf("---------------------------------------\n");
}

void print_optional_header_standard_pe32(OptionalHeaderStandardPE32 *standard_fields)
{
  if (standard_fields == NULL)
  {
    fprintf(stderr, "Invalid Standard PE32 fields pointer.\n");
    return;
  }

  printf("\n");
  printf("Standard PE32 Fields:\n");
  printf("  Magic:                      0x%04x\n", standard_fields->magic);
  printf("  Linker Version:             ");
  print_version(standard_fields->major_linker_version, standard_fields->minor_linker_version, 0);
  printf("  Size of Code:               %u\n", standard_fields->size_of_code);
  printf("  Size of Initialized Data:   %u\n", standard_fields->size_of_initialized_data);
  printf("  Size of Uninitialized Data: %u\n", standard_fields->size_of_bss_data);
  printf("  Address of Entry Point:     %u\n", standard_fields->address_of_entry_point);
  printf("  Base of Code:               %u\n", standard_fields->base_of_code);
  printf("  Base of Data:               %u\n", standard_fields->base_of_data);
}

void print_optional_header_standard_pe32_plus(OptionalHeaderStandardPE32Plus *standard_fields)
{
  if (standard_fields == NULL)
  {
    fprintf(stderr, "Invalid Standard PE32+ fields pointer.\n");
    return;
  }

  printf("\n");
  printf("Standard PE32+ Fields:\n");
  printf("  Magic:                      0x%04x\n", standard_fields->magic);
  printf("  Linker Version:             ");
  print_version(standard_fields->major_linker_version, standard_fields->minor_linker_version, 0);
  printf("  Size of Code:               %u\n", standard_fields->size_of_code);
  printf("  Size of Initialized Data:   %u\n", standard_fields->size_of_initialized_data);
  printf("  Size of Uninitialized Data: %u\n", standard_fields->size_of_bss_data);
  printf("  Address of Entry Point:     %u\n", standard_fields->address_of_entry_point);
  printf("  Base of Code:               %u\n", standard_fields->base_of_code);
}

void print_optional_header_windows_subsystem(uint16_t subsystem);
void print_optional_header_dll_characteristics(uint16_t dll_characteristics, int indent_size);

void print_optional_header_windows_field_pe32(OptionalHeaderWindowsFieldPE32 *windows_fields)
{
  if (windows_fields == NULL)
  {
    fprintf(stderr, "Invalid Windows PE32 fields pointer.\n");
    return;
  }

  printf("\n");
  printf("Windows PE32 Fields:\n");
  printf("  Image Base:                 0x%08x\n", windows_fields->image_base);
  printf("  Section Alignment:          %u\n", windows_fields->section_alignment);
  printf("  File Alignment:             %u\n", windows_fields->file_alignment);
  printf("  OS Version:                 ");
  print_version(windows_fields->major_os_version, windows_fields->minor_os_version, 2);
  printf("  Image Version:              ");
  print_version(windows_fields->major_image_version, windows_fields->minor_image_version, 2);
  printf("  Subsystem Version:          ");
  print_version(windows_fields->major_subsystem_version, windows_fields->minor_subsystem_version, 2);
  printf("  Win32 Version Value:        %u\n", windows_fields->win32_version_value);
  printf("  Size of Image:              %u\n", windows_fields->size_of_image);
  printf("  Size of Headers:            %u\n", windows_fields->size_of_headers);
  printf("  Check Sum:                  0x%08x\n", windows_fields->check_sum);
  print_optional_header_windows_subsystem(windows_fields->subsystem);
  print_optional_header_dll_characteristics(windows_fields->dll_characteristics, 2);
  printf("  Size of Stack Reserve:      %u\n", windows_fields->size_of_stack_reserve);
  printf("  Size of Stack Commit:       %u\n", windows_fields->size_of_stack_commit);
  printf("  Size of Heap Reserve:       %u\n", windows_fields->size_of_heap_reserve);
  printf("  Size of Heap Commit:        %u\n", windows_fields->size_of_heap_commit);
  printf("  Loader Flags:               0x%08x\n", windows_fields->loader_flags);
  printf("  Number of RVA and Sizes:    %u\n", windows_fields->number_of_rva_and_sizes);
}

void print_optional_header_windows_field_pe32_plus(OptionalHeaderWindowsFieldPE32Plus *windows_fields)
{
  if (windows_fields == NULL)
  {
    fprintf(stderr, "Invalid Windows PE32+ fields pointer.\n");
    return;
  }

  printf("\n");
  printf("Windows PE32+ Fields:\n");
  printf("  Image Base:                 0x%016llx\n", (unsigned long long)windows_fields->image_base);
  printf("  Section Alignment:          %u\n", windows_fields->section_alignment);
  printf("  File Alignment:             %u\n", windows_fields->file_alignment);
  printf("  OS Version:                 ");
  print_version(windows_fields->major_os_version, windows_fields->minor_os_version, 0);
  printf("  Image Version:              ");
  print_version(windows_fields->major_image_version, windows_fields->minor_image_version, 0);
  printf("  Subsystem Version:          ");
  print_version(windows_fields->major_subsystem_version, windows_fields->minor_subsystem_version, 0);
  printf("  Win32 Version Value(0):     %u\n", windows_fields->win32_version_value);
  printf("  Size of Image:              %u\n", windows_fields->size_of_image);
  printf("  Size of Headers:            %u\n", windows_fields->size_of_headers);
  printf("  Check Sum:                  0x%08x\n", windows_fields->check_sum);
  printf("  Subsystem:                  ");
  print_optional_header_windows_subsystem(windows_fields->subsystem);
  printf("  DLL Characteristics:\n");
  print_optional_header_dll_characteristics(windows_fields->dll_characteristics, 4);
  printf("  Size of Stack Reserve:      %llu\n", (unsigned long long)windows_fields->size_of_stack_reserve);
  printf("  Size of Stack Commit:       %llu\n", (unsigned long long)windows_fields->size_of_stack_commit);
  printf("  Size of Heap Reserve:       %llu\n", (unsigned long long)windows_fields->size_of_heap_reserve);
  printf("  Size of Heap Commit:        %llu\n", (unsigned long long)windows_fields->size_of_heap_commit);
  printf("  Loader Flags(0):            0x%08x\n", windows_fields->loader_flags);
  printf("  Number of RVA and Sizes:    %u\n", windows_fields->number_of_rva_and_sizes);
}

void print_optional_header_windows_subsystem(uint16_t subsystem)
{
  switch (subsystem)
  {
  case IMAGE_SUBSYSTEM_UNKNOWN:
    printf("Unknown\n");
    break;
  case IMAGE_SUBSYSTEM_NATIVE:
    printf("Device Drivers / Native Process\n");
    break;
  case IMAGE_SUBSYSTEM_WINDOWS_GUI:
    printf("Windows GUI\n");
    break;
  case IMAGE_SUBSYSTEM_WINDOWS_CUI:
    printf("Windows CUI\n");
    break;
  case IMAGE_SUBSYSTEM_OS2_CUI:
    printf("OS/2 CUI\n");
    break;
  case IMAGE_SUBSYSTEM_POSIX_CUI:
    printf("POSIX CUI\n");
    break;
  case IMAGE_SUBSYSTEM_NATIVE_WINDOWS:
    printf("Native Win9x driver\n");
    break;
  case IMAGE_SUBSYSTEM_WINDOWS_CE_GUI:
    printf("Windows CE GUI\n");
    break;
  case IMAGE_SUBSYSTEM_EFI_APPLICATION:
    printf("EFI Application\n");
    break;
  case IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER:
    printf("EFI Driver with Boot Services\n");
    break;
  case IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER:
    printf("EFI Driver with Runtime Services\n");
    break;
  case IMAGE_SUBSYSTEM_EFI_ROM:
    printf("EFI ROM image\n");
    break;
  case IMAGE_SUBSYSTEM_XBOX:
    printf("Xbox\n");
    break;
  case IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION:
    printf("Windows Boot Application\n");
    break;
  default:
    printf("Unknown (0x%04x)\n", subsystem);
    break;
  }
}

void print_optional_header_dll_characteristics(uint16_t dll_characteristics, int indent_size)
{
  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_HIGH_ENTROPY_VA)
  {
    print_characteristic_flag(
        "Image can handle a high entropy 64-bit virtual address space", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_DYNAMIC_BASE)
  {
    print_characteristic_flag(
        "DLL can be relocated at load time", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_FORCE_INTEGRITY)
  {
    print_characteristic_flag(
        "Force code integrity checks", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_NX_COMPAT)
  {
    print_characteristic_flag(
        "Image is NX compatible", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_NO_ISOLATION)
  {
    print_characteristic_flag(
        "Isolation aware, but do not isolate the image", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_NO_SEH)
  {
    print_characteristic_flag(
        "No Structured Exception Handling (SEH) handlers in this image", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_NO_BIND)
  {
    print_characteristic_flag(
        "Do not bind the image", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_APPCONTAINER)
  {
    print_characteristic_flag(
        "Must execute in an AppContainer", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_WDM_DRIVER)
  {
    print_characteristic_flag(
        "WDM driver", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_GUARD_CF)
  {
    print_characteristic_flag(
        "Image supports Control Flow Guard", indent_size);
  }

  if (dll_characteristics & IMAGE_DLL_CHARACTERISTICS_TERMINAL_SERVER_AWARE)
  {
    print_characteristic_flag(
        "Terminal Server aware", indent_size);
  }
}

void print_optional_header_data_directory_entry(
    const char *name,
    OptionalHeaderDataDirectoryEntry entry)
{
  printf("  [%-25s] Virtual Address: 0x%08x, Size: %u\n",
         name,
         entry.virtual_address,
         entry.size);
}

void print_optional_header_data_directory(OptionalHeaderDataDirectory *data_directories)
{
  if (data_directories == NULL)
  {
    fprintf(stderr, "Invalid Data Directories pointer.\n");
    return;
  }

  printf("\n");
  printf("Data Directories:\n");
  print_optional_header_data_directory_entry(
      "Export Table",
      data_directories->export_table);
  print_optional_header_data_directory_entry(
      "Import Table",
      data_directories->import_table);
  print_optional_header_data_directory_entry(
      "Resource Table",
      data_directories->resource_table);
  print_optional_header_data_directory_entry(
      "Exception Table",
      data_directories->exception_table);
  print_optional_header_data_directory_entry(
      "Certificate Table",
      data_directories->certificate_table);
  print_optional_header_data_directory_entry(
      "Base Relocation Table",
      data_directories->base_relocation_table);
  print_optional_header_data_directory_entry(
      "Debug",
      data_directories->debug);
  print_optional_header_data_directory_entry(
      "Architecture(0)",
      data_directories->architecture);
  print_optional_header_data_directory_entry(
      "Global Ptr",
      data_directories->global_ptr);
  print_optional_header_data_directory_entry(
      "TLS Table",
      data_directories->tls_table);
  print_optional_header_data_directory_entry(
      "Load Config Table",
      data_directories->load_config_table);
  print_optional_header_data_directory_entry(
      "Bound Import",
      data_directories->bound_import);
  print_optional_header_data_directory_entry(
      "Import Address Table",
      data_directories->import_address_table);
  print_optional_header_data_directory_entry(
      "Delay Import Descriptor",
      data_directories->delay_import_descriptor);
  print_optional_header_data_directory_entry(
      "CLR Runtime Header",
      data_directories->clr_runtime_header);
  print_optional_header_data_directory_entry(
      "Reserved(0)",
      data_directories->reserved);
}

void print_section_header_image(uint16_t number, SectionHeader *section_header)
{
  if (section_header == NULL)
  {
    fprintf(stderr, "Invalid Section Header pointer.\n");
    return;
  }

  printf("  [%-3u] %-8s 0x%08x 0x%08x 0x%08x 0x%08x",
         number,
         section_header->name,
         section_header->virtual_size,
         section_header->virtual_address,
         section_header->size_of_raw_data,
         section_header->ptr_to_raw_data);

  char flags[5] = {'-', '-', '-', '-', '-'};
  if (section_header->characteristics & IMAGE_SCN_MEM_READ)
  {
    flags[0] = 'r';
  }
  if (section_header->characteristics & IMAGE_SCN_MEM_WRITE)
  {
    flags[1] = 'w';
  }
  if (section_header->characteristics & IMAGE_SCN_MEM_EXECUTE)
  {
    flags[2] = 'x';
  }
  if (section_header->characteristics & IMAGE_SCN_MEM_SHARED)
  {
    flags[3] = 's';
  }
  if (!(section_header->characteristics & IMAGE_SCN_MEM_NOT_PAGED))
  {
    flags[4] = 'p';
  }
  printf(" %c%c%c%c%c\n",
         flags[0],
         flags[1],
         flags[2],
         flags[3],
         flags[4]);
}

void print_section_table(SectionHeader *section_table, uint16_t number_of_sections)
{
  if (section_table == NULL)
  {
    fprintf(stderr, "Invalid Section Table pointer.\n");
    return;
  }

  printf("\n");
  printf("----------- Section Table -----------\n");

  printf("  No    Name     VSize      VAddr      DataSize   DataPtr    Flags\n");
  
  for (uint16_t i = 0; i < number_of_sections; i++)
  {
    print_section_header_image(i + 1, &section_table[i]);
  }

  printf("  Flags: r=readable, w=writable, x=executable, s=shared, p=pageable\n");
  printf("-------------------------------------\n");
}