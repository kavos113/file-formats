#include <fstream>
#include <iostream>

#include "ttf/table_directory.h"
#include "tables/os2.h"
#include "tables/os2_io.h"
#include "tables/head.h"
#include "tables/head_io.h"

int main(int argc, char* argv[])
{
    if (argc < 2)
    {
        std::cerr << "Usage: " << argv[0] << " <font-file.ttf>" << std::endl;
        return 1;
    }

    std::ifstream fontFile(argv[1], std::ios::binary);
    if (!fontFile)
    {
        std::cerr << "Failed to open font file: " << argv[1] << std::endl;
        return 1;
    }

    TTFTableDirectory directory;
    fontFile >> directory;
    std::cout << directory;

    TTFTableRecord os2_record = directory.find_table_record("OS/2");
    fontFile.seekg(os2_record.offset, std::ios::beg);
    OS2Table os2;
    fontFile >> os2;
    std::cout << os2;

    TTFTableRecord head_record = directory.find_table_record("head");
    fontFile.seekg(head_record.offset, std::ios::beg);
    HeadTable head;
    fontFile >> head;
    std::cout << head;

    return 0;
}