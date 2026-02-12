#ifndef HEADER_H
#define HEADER_H

#include "pe.h"

#include <stdio.h>

PEHeader *read_pe_header(FILE *file);

#endif // HEADER_H