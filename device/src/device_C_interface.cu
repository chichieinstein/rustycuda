#include "../include/device_C_interface.cuh"

extern "C"
{
    void* memory_allocate_device(int size)
    {
        void* inter;
        cudaMalloc((void**)&inter, size);
        return inter;
    }

    void memory_deallocate_device(void* inter)
    {
        cudaFree(inter);
    }

    void transfer(void* cpu_arr, void* gpu_arr, int count)
    {
        cudaMemcpy(cpu_arr, gpu_arr, count, cudaMemcpyDeviceToHost);
    }
}