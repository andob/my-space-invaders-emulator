#include "stack.h"
#include "../cpu.h"

void CPUStack::push_byte(const CPU* cpu, const u8 value)
{
    (*cpu->ram)[this->pointer % RAM_SIZE] = value;
    this->pointer--;
}

u8 CPUStack::pop_byte(const CPU* cpu)
{
    this->pointer++;
    return (*cpu->ram)[this->pointer % RAM_SIZE];
}

void CPUStack::push_address(const CPU* cpu, const u16 address)
{
    this->push_byte(cpu, address >> 8);
    this->push_byte(cpu, address & 0xFF);
}

u16 CPUStack::pop_address(const CPU* cpu)
{
    const u8 low = this->pop_byte(cpu);
    const u8 high = this->pop_byte(cpu);
    return static_cast<u16>(high) << 8 | low;
}
