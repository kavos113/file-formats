#ifndef TTFTYPES_H
#define TTFTYPES_H

#include <stdint.h>
#include <stdio.h>

typedef uint8_t uint8;
typedef int8_t int8;
typedef uint16_t uint16;
typedef int16_t int16;
typedef struct
{
  uint8_t data[3];
} uint24;
typedef uint32_t uint32;
typedef int32_t Fixed;
typedef int16 FWORD;
typedef uint16 UFWORD;
typedef int16 F2DOT14;
typedef int64_t LONGDATETIME;
typedef uint32_t Tag;
typedef uint8 Offset8;
typedef uint16 Offset16;
typedef uint24 Offset24;
typedef uint32 Offset32;
typedef uint32 Version16Dot16;

#define f2dot14_to_float(f2dot14) ((float)(f2dot14) / 16384.0f)
#define float_to_f2dot14(f) ((F2DOT14)(f * 16384.0f))

#define make_tag(a, b, c, d) ((Tag)((((uint8_t)(a)) << 24) | (((uint8_t)(b)) << 16) | (((uint8_t)(c)) << 8) | ((uint8_t)(d))))
#define make_tag_str(str) make_tag((str)[0], (str)[1], (str)[2], (str)[3])

#endif // TTFTYPES_H