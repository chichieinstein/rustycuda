#include "cufft.h"

extern "C"
{
    void* memory_allocate_device(int);
    void memory_deallocate_device(void*);
}
