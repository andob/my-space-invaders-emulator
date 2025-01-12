#include "stack.h"

u16 CPUStack::get_pointer() const
{
    return this->pointer;
}

void CPUStack::set_pointer(const u16 address)
{
    this->pointer = address;
}

void CPUStack::push_byte(const u8 value)
{
    this->ram[this->pointer % RAM_SIZE] = value;
    this->pointer--;
}

u8 CPUStack::pop_byte()
{
    this->pointer++;
    return ram[this->pointer % RAM_SIZE];
}

void CPUStack::push_address(const u16 address)
{
    this->push_byte(address >> 8);
    this->push_byte(address & 0xFF);
}

u16 CPUStack::pop_address()
{
    const u8 low = this->pop_byte();
    const u8 high = this->pop_byte();
    return high << 8 | low;
}
