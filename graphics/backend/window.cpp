#include "bindings.h"
#include <stdio.h>

EXTERN int c_hello_world()
{
    printf("Hello World From C++\n");
    return 420;
}

EXTERN int c_start_application()
{
    return 0;
}