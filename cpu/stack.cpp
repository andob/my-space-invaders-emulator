#include "stack.h"
#include "../cpu.h"

u16 CPUStack::get_pointer() const
{
    return this->pointer;
}

void CPUStack::set_pointer(const u16 pointer)
{
    this->pointer = pointer;
    this->max_address = pointer;
    this->min_address = pointer - 0xFF;
}

void CPUStack::push_byte(const CPU* cpu, const u8 value)
{
    this->pointer--;

    //stack overflow!
    if (this->pointer < min_address)
        this->pointer = max_address;

    (*cpu->ram)[this->pointer % RAM_SIZE] = value;
}

u8 CPUStack::pop_byte(const CPU* cpu)
{
    const u8 value = (*cpu->ram)[this->pointer % RAM_SIZE];

    this->pointer++;

    //stack underflow!
    if (this->pointer > max_address)
        this->pointer = min_address;

    return value;
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
    return address_from_high_low(high, low);
}
