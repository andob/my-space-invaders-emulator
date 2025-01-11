#include "cpu.h"
#include <iostream>

CPU::CPU(const vector<u8>& rom_bytes)
{
    //initialise fields
    this->ram.fill(0);
    this->A = this->B = this->C = this->D = this->E = 0;
    this->H = this->L = this->program_counter = 0;
    this->stack_pointer = 0;
    this->flags.as_byte = 0;
}

u8 CPU::next_byte()
{
    if (this->program_counter+1 < RAM_SIZE)
    {
        const u8 opcode = this->ram[this->program_counter];
        this->program_counter++;
        return opcode;
    }

    return 0;
}

void CPU::cpu_tick()
{
    //https://grantmestrength.github.io/RetroComputerInstructionManual/intel8080.html
    const u8 opcode = this->next_byte();
    if (opcode == 0x7F) this->mov(CPURegister::A, CPURegister::A);
    else if (opcode == 0x2F) this->mov(CPURegister::A, CPURegister::B);
    else if (opcode == 0x4F) this->mov(CPURegister::A, CPURegister::C);
    else if (opcode == 0x39) this->mov(CPURegister::A, CPURegister::D);
    else if (opcode == 0x5F) this->mov(CPURegister::A, CPURegister::E);
    else if (opcode == 0x43) this->mov(CPURegister::A, CPURegister::H);
    else if (opcode == 0x6F) this->mov(CPURegister::A, CPURegister::L);
    else if (opcode == 0x4D) this->mov(CPURegister::A, CPURegister::RAM);
    else if (opcode == 0x4E) this->mov(CPURegister::B, CPURegister::A);
    else if (opcode == 0x28) this->mov(CPURegister::B, CPURegister::B);
    else if (opcode == 0x30) this->mov(CPURegister::B, CPURegister::C);
    else if (opcode == 0x32) this->mov(CPURegister::B, CPURegister::D);
    else if (opcode == 0x3A) this->mov(CPURegister::B, CPURegister::E);
    else if (opcode == 0x3C) this->mov(CPURegister::B, CPURegister::H);
    else if (opcode == 0x44) this->mov(CPURegister::B, CPURegister::L);
    else if (opcode == 0x46) this->mov(CPURegister::B, CPURegister::RAM);
    else if (opcode == 0x4F) this->mov(CPURegister::C, CPURegister::A);
    else if (opcode == 0x29) this->mov(CPURegister::C, CPURegister::B);
    else if (opcode == 0x31) this->mov(CPURegister::C, CPURegister::C);
    else if (opcode == 0x33) this->mov(CPURegister::C, CPURegister::D);
    else if (opcode == 0x3B) this->mov(CPURegister::C, CPURegister::E);
    else if (opcode == 0x3D) this->mov(CPURegister::C, CPURegister::H);
    else if (opcode == 0x45) this->mov(CPURegister::C, CPURegister::L);
    else if (opcode == 0x47) this->mov(CPURegister::C, CPURegister::RAM);
    else if (opcode == 0x7A) this->mov(CPURegister::D, CPURegister::A);
    else if (opcode == 0x2A) this->mov(CPURegister::D, CPURegister::B);
    else if (opcode == 0x4A) this->mov(CPURegister::D, CPURegister::C);
    else if (opcode == 0x34) this->mov(CPURegister::D, CPURegister::D);
    else if (opcode == 0x5A) this->mov(CPURegister::D, CPURegister::E);
    else if (opcode == 0x3E) this->mov(CPURegister::D, CPURegister::H);
    else if (opcode == 0x6A) this->mov(CPURegister::D, CPURegister::L);
    else if (opcode == 0x48) this->mov(CPURegister::D, CPURegister::RAM);
    else if (opcode == 0x7B) this->mov(CPURegister::E, CPURegister::A);
    else if (opcode == 0x2B) this->mov(CPURegister::E, CPURegister::B);
    else if (opcode == 0x4B) this->mov(CPURegister::E, CPURegister::C);
    else if (opcode == 0x35) this->mov(CPURegister::E, CPURegister::D);
    else if (opcode == 0x5B) this->mov(CPURegister::E, CPURegister::E);
    else if (opcode == 0x3F) this->mov(CPURegister::E, CPURegister::H);
    else if (opcode == 0x6B) this->mov(CPURegister::E, CPURegister::L);
    else if (opcode == 0x49) this->mov(CPURegister::E, CPURegister::RAM);
    else if (opcode == 0x7C) this->mov(CPURegister::H, CPURegister::A);
    else if (opcode == 0x2C) this->mov(CPURegister::H, CPURegister::B);
    else if (opcode == 0x4C) this->mov(CPURegister::H, CPURegister::C);
    else if (opcode == 0x36) this->mov(CPURegister::H, CPURegister::D);
    else if (opcode == 0x5C) this->mov(CPURegister::H, CPURegister::E);
    else if (opcode == 0x40) this->mov(CPURegister::H, CPURegister::H);
    else if (opcode == 0x6C) this->mov(CPURegister::H, CPURegister::L);
    else if (opcode == 0x4A) this->mov(CPURegister::H, CPURegister::RAM);
    else if (opcode == 0x7D) this->mov(CPURegister::L, CPURegister::A);
    else if (opcode == 0x2D) this->mov(CPURegister::L, CPURegister::B);
    else if (opcode == 0x4D) this->mov(CPURegister::L, CPURegister::C);
    else if (opcode == 0x37) this->mov(CPURegister::L, CPURegister::D);
    else if (opcode == 0x5D) this->mov(CPURegister::L, CPURegister::E);
    else if (opcode == 0x41) this->mov(CPURegister::L, CPURegister::H);
    else if (opcode == 0x6D) this->mov(CPURegister::L, CPURegister::L);
    else if (opcode == 0x4B) this->mov(CPURegister::L, CPURegister::RAM);
    else if (opcode == 0x7E) this->mov(CPURegister::RAM, CPURegister::A);
    else if (opcode == 0x2E) this->mov(CPURegister::RAM, CPURegister::B);
    else if (opcode == 0x4E) this->mov(CPURegister::RAM, CPURegister::C);
    else if (opcode == 0x38) this->mov(CPURegister::RAM, CPURegister::D);
    else if (opcode == 0x5E) this->mov(CPURegister::RAM, CPURegister::E);
    else if (opcode == 0x42) this->mov(CPURegister::RAM, CPURegister::H);
    else if (opcode == 0x6E) this->mov(CPURegister::RAM, CPURegister::L);
    else if (opcode == 0x3E) this->mvi(this->next_byte(), CPURegister::A);
    else if (opcode == 0x06) this->mvi(this->next_byte(), CPURegister::B);
    else if (opcode == 0x0E) this->mvi(this->next_byte(), CPURegister::C);
    else if (opcode == 0x10) this->mvi(this->next_byte(), CPURegister::D);
    else if (opcode == 0x1E) this->mvi(this->next_byte(), CPURegister::E);
    else if (opcode == 0x1A) this->mvi(this->next_byte(), CPURegister::H);
    else if (opcode == 0x2E) this->mvi(this->next_byte(), CPURegister::L);
    else if (opcode == 0x24) this->mvi(this->next_byte(), CPURegister::RAM);
    else if (opcode == 0xA7) this->ana(CPURegister::A);
    else if (opcode == 0xA0) this->ana(CPURegister::B);
    else if (opcode == 0xA1) this->ana(CPURegister::C);
    else if (opcode == 0xA2) this->ana(CPURegister::D);
    else if (opcode == 0xA3) this->ana(CPURegister::E);
    else if (opcode == 0xA4) this->ana(CPURegister::H);
    else if (opcode == 0xA5) this->ana(CPURegister::L);
    else if (opcode == 0xA6) this->ana(CPURegister::RAM);
    else if (opcode == 0xE6) this->ani(this->next_byte());
    else if (opcode == 0xAF) this->xra(CPURegister::A);
    else if (opcode == 0xA8) this->xra(CPURegister::B);
    else if (opcode == 0xA9) this->xra(CPURegister::C);
    else if (opcode == 0xAA) this->xra(CPURegister::D);
    else if (opcode == 0xAB) this->xra(CPURegister::E);
    else if (opcode == 0xAC) this->xra(CPURegister::H);
    else if (opcode == 0xAD) this->xra(CPURegister::L);
    else if (opcode == 0xAE) this->xra(CPURegister::RAM);
    else if (opcode == 0xEE) this->xri(this->next_byte());
    else if (opcode == 0xB7) this->ora(CPURegister::A);
    else if (opcode == 0xB0) this->ora(CPURegister::B);
    else if (opcode == 0xB1) this->ora(CPURegister::C);
    else if (opcode == 0xB2) this->ora(CPURegister::D);
    else if (opcode == 0xB3) this->ora(CPURegister::E);
    else if (opcode == 0xB4) this->ora(CPURegister::H);
    else if (opcode == 0xB5) this->ora(CPURegister::L);
    else if (opcode == 0xB6) this->ora(CPURegister::RAM);
    else if (opcode == 0xF6) this->ori(this->next_byte());
    else if (opcode == 0x0F) this->rrc();
    else if (opcode == 0x1F) this->rar();
    else if (opcode == 0x07) this->rlc();
    else if (opcode == 0x11) this->ral();
    else if (opcode == 0x3C) this->inr(CPURegister::A);
    else if (opcode == 0x04) this->inr(CPURegister::B);
    else if (opcode == 0x0C) this->inr(CPURegister::C);
    else if (opcode == 0x0E) this->inr(CPURegister::D);
    else if (opcode == 0x1C) this->inr(CPURegister::E);
    else if (opcode == 0x18) this->inr(CPURegister::H);
    else if (opcode == 0x2C) this->inr(CPURegister::L);
    else if (opcode == 0x22) this->inr(CPURegister::RAM);
    else if (opcode == 0x03) this->inx(CPURegister::BC);
    else if (opcode == 0x0D) this->inx(CPURegister::DE);
    else if (opcode == 0x17) this->inx(CPURegister::HL);
    else if (opcode == 0x21) this->inx(CPURegister::SP);
    else if (opcode == 0x3D) this->dcr(CPURegister::A);
    else if (opcode == 0x05) this->dcr(CPURegister::B);
    else if (opcode == 0x0D) this->dcr(CPURegister::C);
    else if (opcode == 0x0F) this->dcr(CPURegister::D);
    else if (opcode == 0x1D) this->dcr(CPURegister::E);
    else if (opcode == 0x19) this->dcr(CPURegister::H);
    else if (opcode == 0x2D) this->dcr(CPURegister::L);
    else if (opcode == 0x23) this->dcr(CPURegister::RAM);
    else if (opcode == 0x0B) this->dcx(CPURegister::BC);
    else if (opcode == 0x1B) this->dcx(CPURegister::DE);
    else if (opcode == 0x2B) this->dcx(CPURegister::HL);
    else if (opcode == 0x3B) this->dcx(CPURegister::SP);
    else if (opcode) cout << "Opcode not implemented " << hex << opcode << "!\n";
}

u16 CPU::read_register(const CPURegister cpu_register)
{
    switch (cpu_register)
    {
        case CPURegister::A: return this->A;
        case CPURegister::B: return this->B;
        case CPURegister::C: return this->C;
        case CPURegister::D: return this->D;
        case CPURegister::E: return this->E;
        case CPURegister::H: return this->H;
        case CPURegister::L: return this->L;
        case CPURegister::BC: return this->B << 8 | this->C;
        case CPURegister::DE: return this->D << 8 | this->E;
        case CPURegister::HL: return this->H << 8 | this->L;
        case CPURegister::SP: return this->stack_pointer;
        case CPURegister::RAM:
        {
            const u16 address = this->H << 8 | this->L;
            return this->ram[address % RAM_SIZE];
        }
    }
}

void CPU::write_register(const CPURegister cpu_register, const u16 value)
{
    switch (cpu_register)
    {
        case CPURegister::A: this->A = value; break;
        case CPURegister::B: this->B = value; break;
        case CPURegister::C: this->C = value; break;
        case CPURegister::D: this->D = value; break;
        case CPURegister::E: this->E = value; break;
        case CPURegister::H: this->H = value; break;
        case CPURegister::L: this->L = value; break;
        case CPURegister::BC: this->B = value >> 8; this->C = value & 0xFF; break;
        case CPURegister::DE: this->D = value >> 8; this->E = value & 0xFF; break;
        case CPURegister::HL: this->H = value >> 8; this->L = value & 0xFF; break;
        case CPURegister::SP: this->stack_pointer = value; break;
        case CPURegister::RAM:
        {
            const u16 address = this->H << 8 | this->L;
            this->ram[address % RAM_SIZE] = value;
            break;
        }
    }
}

void CPU::mov(const CPURegister from, const CPURegister to)
{
    //copy value of from register into to register
    const u16 value = this->read_register(from);
    this->write_register(to, value);
}

void CPU::mvi(const u16 value, const CPURegister to)
{
    //copy value into to register
    this->write_register(to, value);
}

void CPU::ana(const CPURegister target)
{
    //logical AND value with Accumulator
    this->ani(this->read_register(target));
}

void CPU::ani(const u16 value)
{
    //logical AND value with Accumulator
    const u16 new_value = this->A & value;
    this->A = new_value;
    this->update_arithmetic_flags(new_value);
}

void CPU::xra(const CPURegister target)
{
    //logical XOR value with Accumulator
    this->xri(this->read_register(target));
}

void CPU::xri(const u16 value)
{
    //logical XOR value with Accumulator
    const u16 new_value = this->A ^ value;
    this->A = new_value;
    this->update_arithmetic_flags(new_value);
}

void CPU::ora(const CPURegister target)
{
    //logical OR value with Accumulator
    this->ori(this->read_register(target));
}

void CPU::ori(const u16 value)
{
    //logical OR value with Accumulator
    const u16 new_value = this->A | value;
    this->A = new_value;
    this->update_arithmetic_flags(new_value);
}

void CPU::rrc()
{
    //shift right
    const u8 bit_to_carry = this->A & 1;
    const u8 bit_to_push = bit_to_carry;
    this->flags.as_bits.carry = bit_to_carry == 1;
    this->A = (bit_to_push << 7) | (this->A >> 1);

}

void CPU::rar()
{
    //shift right
    const u8 bit_to_carry = this->A & 1;
    const u8 bit_to_push = this->flags.as_bits.carry;
    this->flags.as_bits.carry = bit_to_carry == 1;
    this->A = (bit_to_push << 7) | (this->A >> 1);
}

void CPU::rlc()
{
    //shift left
    const u8 bit_to_carry = this->A >> 7;
    const u8 bit_to_push = bit_to_carry;
    this->flags.as_bits.carry = bit_to_carry == 1;
    this->A = (this->A << 1) | bit_to_push;
}

void CPU::ral()
{
    //shift left
    const u8 bit_to_carry = this->A >> 7;
    const u8 bit_to_push = this->flags.as_bits.carry;
    this->flags.as_bits.carry = bit_to_carry == 1;
    this->A = (this->A << 1) | bit_to_push;
}

void CPU::inr(const CPURegister target)
{
    //increment register value
    const u8 old_value = this->read_register(target);
    const u8 new_value = old_value + 1;
    this->write_register(target, new_value);
    this->update_arithmetic_flags(new_value);
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = old_value == 0xFF;
}

void CPU::inx(const CPURegister target)
{
    //increment register value
    const u16 old_value = this->read_register(target);
    const u16 new_value = old_value + 1;
    this->write_register(target, new_value);
    this->update_arithmetic_flags(new_value);
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = old_value == 0xFFFF;
}

void CPU::dcr(const CPURegister target)
{
    //decrement register value
    const u8 old_value = this->read_register(target);
    const u8 new_value = old_value - 1;
    this->write_register(target, new_value);
    this->update_arithmetic_flags(new_value);
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = old_value == 0;
}

void CPU::dcx(const CPURegister target)
{
    //decrement register value
    const u16 old_value = this->read_register(target);
    const u16 new_value = old_value - 1;
    this->write_register(target, new_value);
    this->update_arithmetic_flags(new_value);
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = old_value == 0;
}

void CPU::update_arithmetic_flags(const u16 value)
{
    this->flags.as_bits.zero = (value & 0xFF) == 0;
    this->flags.as_bits.negative = (value & 0xFF) > 0x7F;
    this->flags.as_bits.even = (value & 0xFF) % 2 == 0;
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = value > 0xFF;
}

void CPU::run()
{
    this->cpu_tick();
}
