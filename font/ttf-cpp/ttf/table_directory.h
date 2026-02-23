#ifndef TTF_TABLE_DIRECTORY_H
#define TTF_TABLE_DIRECTORY_H

#include "types.h"
#include "util/util.h"

#include <vector>
#include <iostream>
#include <format>

struct TTFTableRecord
{
    Tag tag;
    uint32 checksum;
    Offset32 offset;
    uint32 length;
};

template<typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, TTFTableRecord& record)
{
    is.read(reinterpret_cast<char*>(&record), sizeof(TTFTableRecord));

    endian_swap(&record.tag, sizeof(Tag));
    endian_swap(&record.checksum, sizeof(uint32));
    endian_swap(&record.offset, sizeof(Offset32));
    endian_swap(&record.length, sizeof(uint32));
    return is;
}

struct TTFTableDirectory
{
    uint32 sfnt_version;
    uint16 num_tables;
    uint16 search_range;
    uint16 entry_selector;
    uint16 range_shift;
    std::vector<TTFTableRecord> table_records;

    const TTFTableRecord& find_table_record(const std::string& tag) const
    {
        Tag tagValue = make_tag_str(tag);
        for (const auto& record : table_records)
        {
            if (record.tag == tagValue)
            {
                return record;
            }
        }
        throw std::runtime_error("Table not found: " + tag);
    }
};

template<typename CharT, typename Traits>
std::basic_istream<CharT, Traits>&
    operator>>(std::basic_istream<CharT, Traits>& is, TTFTableDirectory& directory)
{
    is.read(reinterpret_cast<char*>(&directory.sfnt_version), sizeof(uint32));
    is.read(reinterpret_cast<char*>(&directory.num_tables), sizeof(uint16));
    is.read(reinterpret_cast<char*>(&directory.search_range), sizeof(uint16));
    is.read(reinterpret_cast<char*>(&directory.entry_selector), sizeof(uint16));
    is.read(reinterpret_cast<char*>(&directory.range_shift), sizeof(uint16));

    endian_swap(&directory.sfnt_version, sizeof(uint32));
    endian_swap(&directory.num_tables, sizeof(uint16));
    endian_swap(&directory.search_range, sizeof(uint16));
    endian_swap(&directory.entry_selector, sizeof(uint16));
    endian_swap(&directory.range_shift, sizeof(uint16));

    directory.table_records.resize(directory.num_tables);
    for (uint16 i = 0; i < directory.num_tables; ++i)
    {
        is >> directory.table_records[i];
    }
    return is;
}

template<typename CharT, typename Traits>
std::basic_ostream<CharT, Traits>&
    operator<<(std::basic_ostream<CharT, Traits>& os, const TTFTableDirectory& directory)
{
    std::string versionStr;
    if (directory.sfnt_version == 0x00010000)
    {
        versionStr = "TrueType";
    }
    else if (directory.sfnt_version == 0x4F54544F) // 'OTTO'
    {
        versionStr = "OpenType";
    }

    os << std::format("SFNT Version:     0x{:08x} ({})\n", directory.sfnt_version, versionStr);
    os << std::format("Number of Tables: {}\n", directory.num_tables);
    os << std::format("Search Range:     {}\n", directory.search_range);
    os << std::format("Entry Selector:   {}\n", directory.entry_selector);
    os << std::format("Range Shift:      {}\n", directory.range_shift);

    os << "\nTable Records\n";
    os << "Tag   Checksum   Offset     Length    \n";
    for (const auto& record : directory.table_records)
    {
        os << std::format("{:<4} 0x{:08x} 0x{:08x} 0x{:08x}\n",
                          tag_to_string(record.tag), record.checksum, record.offset, record.length);
    }
    return os;
}

#endif // TTF_TABLE_DIRECTORY_H