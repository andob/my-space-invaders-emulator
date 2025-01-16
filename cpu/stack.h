#ifndef STACK_H
#define STACK_H

#include "../constants.h"
#include <memory>

using namespace std;

class CPU;
class CPUStack
{
    u16 pointer;
    u16 max_address;
    u16 min_address;

public:
    explicit CPUStack() : pointer(0xFFFF), max_address(0xFFFF), min_address(0x0000) {}

    u16 get_pointer() const;
    void set_pointer(u16 pointer);

    void push_byte(const CPU* cpu, u8 value);
    u8 pop_byte(const CPU* cpu);

    void push_address(const CPU* cpu, u16 address);
    u16 pop_address(const CPU* cpu);
};

#endif //STACK_H
