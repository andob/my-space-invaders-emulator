#ifndef STACK_H
#define STACK_H

#include "../constants.h"
#include <array>
#include <memory>

using namespace std;

class CPU;
class CPUStack
{
public:
    u16 pointer;

    explicit CPUStack() : pointer(0) {}

    void push_byte(const CPU* cpu, u8 value);
    u8 pop_byte(const CPU* cpu);

    void push_address(const CPU* cpu, u16 address);
    u16 pop_address(const CPU* cpu);
};

#endif //STACK_H
