#ifndef STACK_H
#define STACK_H

#include "../constants.h"
#include <array>

using namespace std;

class CPUStack
{
    array<u8, RAM_SIZE>& ram;
    u16 pointer;

public:
    explicit CPUStack(array<u8, RAM_SIZE>& ram) : ram(ram), pointer(0) {}

    u16 get_pointer() const;
    void set_pointer(u16 address);

    void push_byte(u8 value);
    u8 pop_byte();

    void push_address(u16 address);
    u16 pop_address();
};

#endif //STACK_H
