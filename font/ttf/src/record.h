#ifndef RECORD_H
#define RECORD_H

#include "ttftypes.h"
#include "ttf.h"

#include <stdio.h>

TTFTableDirectory *read_ttf_table_directory(FILE *file);

#endif // RECORD_H