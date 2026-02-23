#ifndef UTIL_UTIL_H
#define UTIL_UTIL_H

#include <cstdint>

inline void endian_swap(void *value, size_t size)
{
    auto bytes = static_cast<uint8_t*>(value);
    for (size_t i = 0; i < size / 2; i++)
    {
        uint8_t temp = bytes[i];
        bytes[i] = bytes[size - 1 - i];
        bytes[size - 1 - i] = temp;
    }
}

#endif // UTIL_UTIL_H