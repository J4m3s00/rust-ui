#include "CLib.h"
#include <stdio.h>

extern "C" int c_hello_world()
{
    printf("Hello World From C++\n");
    return 420;
}