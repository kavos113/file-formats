#ifndef TTF_TYPES_H
#define TTF_TYPES_H

#include <cstdint>
#include <string>
#include <stdexcept>
#include <format>
#include <chrono>

using uint8 = uint8_t;
using int8 = int8_t;
using uint16 = uint16_t;
using int16 = int16_t;
using uint24 = struct
{
    uint8_t data[3];
};
using uint32 = uint32_t;
using int32 = int32_t;
using Fixed = int32_t;
using FWORD = int16_t;
using UFWORD = uint16_t;
using F2DOT14 = int16_t;
using LONGDATETIME = int64_t;
using Tag = uint32_t;
using Offset8 = uint8_t;
using Offset16 = uint16_t;
using Offset24 = uint24;
using Offset32 = uint32_t;
using Version16Dot16 = uint32_t;

inline float f2dot14_to_float(F2DOT14 f2dot14)
{
    return static_cast<float>(f2dot14) / 16384.0f;
}

inline F2DOT14 float_to_f2dot14(float f)
{
    return static_cast<F2DOT14>(f * 16384.0f);
}

inline Tag make_tag(char a, char b, char c, char d)
{
    return (static_cast<uint8_t>(a) << 24) |
           (static_cast<uint8_t>(b) << 16) |
           (static_cast<uint8_t>(c) << 8) |
           static_cast<uint8_t>(d);
}

inline Tag make_tag_str(const std::string& str)
{
    if (str.size() != 4)
    {
        throw std::invalid_argument("Tag string must be exactly 4 characters long");
    }
    return make_tag(str[0], str[1], str[2], str[3]);
}

inline std::string tag_to_string(Tag tag)
{
    return std::string{
        static_cast<char>((tag >> 24) & 0xFF),
        static_cast<char>((tag >> 16) & 0xFF),
        static_cast<char>((tag >> 8) & 0xFF),
        static_cast<char>(tag & 0xFF)
    };
}

inline std::string version16dot16_to_string(Version16Dot16 version)
{
    return std::format("{}.{}", version >> 16, version & 0xFFFF);
}

inline std::string longdatetime_to_string(LONGDATETIME datetime)
{
    using namespace std::chrono;

    constexpr int64_t SECONDS_FROM_1904_TO_1970 = 2082844800;
    sys_seconds tp{seconds{datetime - SECONDS_FROM_1904_TO_1970}};
    zoned_time zt{current_zone(), tp};

    return std::format("{:%Y-%m-%d %H:%M:%S}", zt);
}

#endif // TTF_TYPES_H