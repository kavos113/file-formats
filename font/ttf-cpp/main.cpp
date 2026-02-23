#include <fstream>
#include <iostream>

#include "ttf/table_directory.h"

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

    return 0;
}