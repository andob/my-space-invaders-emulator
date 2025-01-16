#include "cpu.h"
#include <iostream>

CPU::CPU()
{
    this->A = this->B = this->C = this->D = this->E = 0;
    this->H = this->L = this->program_counter = 0;
    this->flags.as_byte = 0;
    this->shift_register = 0;
    this->shift_register_offset = 0;
    this->are_interrupts_enabled = true;
    this->ram = make_unique<array<u8, RAM_SIZE>>();
    this->ram->fill(0);
}

u8 CPU::next_byte()
{
    if (this->program_counter+1 < RAM_SIZE)
    {
        const u8 opcode = (*this->ram)[this->program_counter];
        this->program_counter++;
        return opcode;
    }

    return 0;
}

u16 CPU::next_address()
{
    const u8 low = this->next_byte();
    const u8 high = this->next_byte();
    return address_from_high_low(high, low);
}

u8 CPU::tick()
{
    const u8 read_opcode = this->next_byte();

    #define execute_opcode(opcode, cycles, code) if (read_opcode == opcode) { code; return cycles; }
    execute_opcode(0x7F,  5, this->mov(CPURegister::A, CPURegister::A));
    execute_opcode(0x47,  5, this->mov(CPURegister::A, CPURegister::B));
    execute_opcode(0x4f,  5, this->mov(CPURegister::A, CPURegister::C));
    execute_opcode(0x57,  5, this->mov(CPURegister::A, CPURegister::D));
    execute_opcode(0x5f,  5, this->mov(CPURegister::A, CPURegister::E));
    execute_opcode(0x67,  5, this->mov(CPURegister::A, CPURegister::H));
    execute_opcode(0x6f,  5, this->mov(CPURegister::A, CPURegister::L));
    execute_opcode(0x77,  7, this->mov(CPURegister::A, CPURegister::RAM));
    execute_opcode(0x78,  5, this->mov(CPURegister::B, CPURegister::A));
    execute_opcode(0x40,  5, this->mov(CPURegister::B, CPURegister::B));
    execute_opcode(0x48,  5, this->mov(CPURegister::B, CPURegister::C));
    execute_opcode(0x50,  5, this->mov(CPURegister::B, CPURegister::D));
    execute_opcode(0x58,  5, this->mov(CPURegister::B, CPURegister::E));
    execute_opcode(0x60,  5, this->mov(CPURegister::B, CPURegister::H));
    execute_opcode(0x68,  5, this->mov(CPURegister::B, CPURegister::L));
    execute_opcode(0x70,  7, this->mov(CPURegister::B, CPURegister::RAM));
    execute_opcode(0x79,  5, this->mov(CPURegister::C, CPURegister::A));
    execute_opcode(0x41,  5, this->mov(CPURegister::C, CPURegister::B));
    execute_opcode(0x49,  5, this->mov(CPURegister::C, CPURegister::C));
    execute_opcode(0x51,  5, this->mov(CPURegister::C, CPURegister::D));
    execute_opcode(0x59,  5, this->mov(CPURegister::C, CPURegister::E));
    execute_opcode(0x61,  5, this->mov(CPURegister::C, CPURegister::H));
    execute_opcode(0x69,  5, this->mov(CPURegister::C, CPURegister::L));
    execute_opcode(0x71,  7, this->mov(CPURegister::C, CPURegister::RAM));
    execute_opcode(0x7a,  5, this->mov(CPURegister::D, CPURegister::A));
    execute_opcode(0x42,  5, this->mov(CPURegister::D, CPURegister::B));
    execute_opcode(0x4a,  5, this->mov(CPURegister::D, CPURegister::C));
    execute_opcode(0x52,  5, this->mov(CPURegister::D, CPURegister::D));
    execute_opcode(0x5a,  5, this->mov(CPURegister::D, CPURegister::E));
    execute_opcode(0x62,  5, this->mov(CPURegister::D, CPURegister::H));
    execute_opcode(0x6a,  5, this->mov(CPURegister::D, CPURegister::L));
    execute_opcode(0x72,  7, this->mov(CPURegister::D, CPURegister::RAM));
    execute_opcode(0x7b,  5, this->mov(CPURegister::E, CPURegister::A));
    execute_opcode(0x43,  5, this->mov(CPURegister::E, CPURegister::B));
    execute_opcode(0x4b,  5, this->mov(CPURegister::E, CPURegister::C));
    execute_opcode(0x53,  5, this->mov(CPURegister::E, CPURegister::D));
    execute_opcode(0x5b,  5, this->mov(CPURegister::E, CPURegister::E));
    execute_opcode(0x63,  5, this->mov(CPURegister::E, CPURegister::H));
    execute_opcode(0x6b,  5, this->mov(CPURegister::E, CPURegister::L));
    execute_opcode(0x73,  7, this->mov(CPURegister::E, CPURegister::RAM));
    execute_opcode(0x7c,  5, this->mov(CPURegister::H, CPURegister::A));
    execute_opcode(0x44,  5, this->mov(CPURegister::H, CPURegister::B));
    execute_opcode(0x4c,  5, this->mov(CPURegister::H, CPURegister::C));
    execute_opcode(0x54,  5, this->mov(CPURegister::H, CPURegister::D));
    execute_opcode(0x5c,  5, this->mov(CPURegister::H, CPURegister::E));
    execute_opcode(0x64,  5, this->mov(CPURegister::H, CPURegister::H));
    execute_opcode(0x6c,  5, this->mov(CPURegister::H, CPURegister::L));
    execute_opcode(0x74,  7, this->mov(CPURegister::H, CPURegister::RAM));
    execute_opcode(0x7d,  5, this->mov(CPURegister::L, CPURegister::A));
    execute_opcode(0x45,  5, this->mov(CPURegister::L, CPURegister::B));
    execute_opcode(0x4d,  5, this->mov(CPURegister::L, CPURegister::C));
    execute_opcode(0x55,  5, this->mov(CPURegister::L, CPURegister::D));
    execute_opcode(0x5d,  5, this->mov(CPURegister::L, CPURegister::E));
    execute_opcode(0x65,  5, this->mov(CPURegister::L, CPURegister::H));
    execute_opcode(0x6d,  5, this->mov(CPURegister::L, CPURegister::L));
    execute_opcode(0x75,  7, this->mov(CPURegister::L, CPURegister::RAM));
    execute_opcode(0x7e,  7, this->mov(CPURegister::RAM, CPURegister::A));
    execute_opcode(0x46,  7, this->mov(CPURegister::RAM, CPURegister::B));
    execute_opcode(0x4e,  7, this->mov(CPURegister::RAM, CPURegister::C));
    execute_opcode(0x56,  7, this->mov(CPURegister::RAM, CPURegister::D));
    execute_opcode(0x5e,  7, this->mov(CPURegister::RAM, CPURegister::E));
    execute_opcode(0x66,  7, this->mov(CPURegister::RAM, CPURegister::H));
    execute_opcode(0x6e,  7, this->mov(CPURegister::RAM, CPURegister::L));
    execute_opcode(0x3E,  7, this->mvi(this->next_byte(), CPURegister::A));
    execute_opcode(0x06,  7, this->mvi(this->next_byte(), CPURegister::B));
    execute_opcode(0x0E,  7, this->mvi(this->next_byte(), CPURegister::C));
    execute_opcode(0x16,  7, this->mvi(this->next_byte(), CPURegister::D));
    execute_opcode(0x1E,  7, this->mvi(this->next_byte(), CPURegister::E));
    execute_opcode(0x26,  7, this->mvi(this->next_byte(), CPURegister::H));
    execute_opcode(0x2E,  7, this->mvi(this->next_byte(), CPURegister::L));
    execute_opcode(0x36, 10, this->mvi(this->next_byte(), CPURegister::RAM));
    execute_opcode(0xa7,  4, this->ana(CPURegister::A));
    execute_opcode(0xa0,  4, this->ana(CPURegister::B));
    execute_opcode(0xa1,  4, this->ana(CPURegister::C));
    execute_opcode(0xa2,  4, this->ana(CPURegister::D));
    execute_opcode(0xa3,  4, this->ana(CPURegister::E));
    execute_opcode(0xa4,  4, this->ana(CPURegister::H));
    execute_opcode(0xa5,  4, this->ana(CPURegister::L));
    execute_opcode(0xa6,  7, this->ana(CPURegister::RAM));
    execute_opcode(0xE6,  7, this->ani(this->next_byte()));
    execute_opcode(0xaf,  4, this->xra(CPURegister::A));
    execute_opcode(0xa8,  4, this->xra(CPURegister::B));
    execute_opcode(0xa9,  4, this->xra(CPURegister::C));
    execute_opcode(0xaa,  4, this->xra(CPURegister::D));
    execute_opcode(0xab,  4, this->xra(CPURegister::E));
    execute_opcode(0xac,  4, this->xra(CPURegister::H));
    execute_opcode(0xad,  4, this->xra(CPURegister::L));
    execute_opcode(0xae,  7, this->xra(CPURegister::RAM));
    execute_opcode(0xEE,  7, this->xri(this->next_byte()));
    execute_opcode(0xb7,  4, this->ora(CPURegister::A));
    execute_opcode(0xb0,  4, this->ora(CPURegister::B));
    execute_opcode(0xb1,  4, this->ora(CPURegister::C));
    execute_opcode(0xb2,  4, this->ora(CPURegister::D));
    execute_opcode(0xb3,  4, this->ora(CPURegister::E));
    execute_opcode(0xb4,  4, this->ora(CPURegister::H));
    execute_opcode(0xb5,  4, this->ora(CPURegister::L));
    execute_opcode(0xb6,  7, this->ora(CPURegister::RAM));
    execute_opcode(0xF6,  7, this->ori(this->next_byte()));
    execute_opcode(0x0F,  4, this->rrc());
    execute_opcode(0x1F,  4, this->rar());
    execute_opcode(0x07,  4, this->rlc());
    execute_opcode(0x17,  4, this->ral());
    execute_opcode(0x3C,  5, this->inr(CPURegister::A));
    execute_opcode(0x04,  5, this->inr(CPURegister::B));
    execute_opcode(0x0C,  5, this->inr(CPURegister::C));
    execute_opcode(0x14,  5, this->inr(CPURegister::D));
    execute_opcode(0x1C,  5, this->inr(CPURegister::E));
    execute_opcode(0x24,  5, this->inr(CPURegister::H));
    execute_opcode(0x2C,  5, this->inr(CPURegister::L));
    execute_opcode(0x34, 10, this->inr(CPURegister::RAM));
    execute_opcode(0x03,  5, this->inx(CPURegister::BC));
    execute_opcode(0x13,  5, this->inx(CPURegister::DE));
    execute_opcode(0x23,  5, this->inx(CPURegister::HL));
    execute_opcode(0x33,  5, this->inx(CPURegister::StackPointer));
    execute_opcode(0x3D,  5, this->dcr(CPURegister::A));
    execute_opcode(0x05,  5, this->dcr(CPURegister::B));
    execute_opcode(0x0D,  5, this->dcr(CPURegister::C));
    execute_opcode(0x15,  5, this->dcr(CPURegister::D));
    execute_opcode(0x1D,  5, this->dcr(CPURegister::E));
    execute_opcode(0x25,  5, this->dcr(CPURegister::H));
    execute_opcode(0x2D,  5, this->dcr(CPURegister::L));
    execute_opcode(0x35, 10, this->dcr(CPURegister::RAM));
    execute_opcode(0x0B,  5, this->dcx(CPURegister::BC));
    execute_opcode(0x1B,  5, this->dcx(CPURegister::DE));
    execute_opcode(0x2B,  5, this->dcx(CPURegister::HL));
    execute_opcode(0x3B,  5, this->dcx(CPURegister::StackPointer));
    execute_opcode(0x87,  4, this->add(CPURegister::A));
    execute_opcode(0x80,  4, this->add(CPURegister::B));
    execute_opcode(0x81,  4, this->add(CPURegister::C));
    execute_opcode(0x82,  4, this->add(CPURegister::D));
    execute_opcode(0x83,  4, this->add(CPURegister::E));
    execute_opcode(0x84,  4, this->add(CPURegister::H));
    execute_opcode(0x85,  4, this->add(CPURegister::L));
    execute_opcode(0x86,  7, this->add(CPURegister::RAM));
    execute_opcode(0xC6,  7, this->adi(this->next_byte()));
    execute_opcode(0x8f,  4, this->adc(CPURegister::A));
    execute_opcode(0x88,  4, this->adc(CPURegister::B));
    execute_opcode(0x89,  4, this->adc(CPURegister::C));
    execute_opcode(0x8a,  4, this->adc(CPURegister::D));
    execute_opcode(0x8b,  4, this->adc(CPURegister::E));
    execute_opcode(0x8c,  4, this->adc(CPURegister::H));
    execute_opcode(0x8d,  4, this->adc(CPURegister::L));
    execute_opcode(0x8e,  7, this->adc(CPURegister::RAM));
    execute_opcode(0xCE,  7, this->aci(this->next_byte()));
    execute_opcode(0x97,  4, this->sub(CPURegister::A));
    execute_opcode(0x90,  4, this->sub(CPURegister::B));
    execute_opcode(0x91,  4, this->sub(CPURegister::C));
    execute_opcode(0x92,  4, this->sub(CPURegister::D));
    execute_opcode(0x93,  4, this->sub(CPURegister::E));
    execute_opcode(0x94,  4, this->sub(CPURegister::H));
    execute_opcode(0x95,  4, this->sub(CPURegister::L));
    execute_opcode(0x96,  7, this->sub(CPURegister::RAM));
    execute_opcode(0xD6,  7, this->sui(this->next_byte()));
    execute_opcode(0x9f,  4, this->sbb(CPURegister::A));
    execute_opcode(0x98,  4, this->sbb(CPURegister::B));
    execute_opcode(0x99,  4, this->sbb(CPURegister::C));
    execute_opcode(0x9a,  4, this->sbb(CPURegister::D));
    execute_opcode(0x9b,  4, this->sbb(CPURegister::E));
    execute_opcode(0x9c,  4, this->sbb(CPURegister::H));
    execute_opcode(0x9d,  4, this->sbb(CPURegister::L));
    execute_opcode(0x9e,  7, this->sbb(CPURegister::RAM));
    execute_opcode(0xDE,  7, this->sbi(this->next_byte()));
    execute_opcode(0xbf,  4, this->cmp(CPURegister::A));
    execute_opcode(0xb8,  4, this->cmp(CPURegister::B));
    execute_opcode(0xb9,  4, this->cmp(CPURegister::C));
    execute_opcode(0xba,  4, this->cmp(CPURegister::D));
    execute_opcode(0xbb,  4, this->cmp(CPURegister::E));
    execute_opcode(0xbc,  4, this->cmp(CPURegister::H));
    execute_opcode(0xbd,  4, this->cmp(CPURegister::L));
    execute_opcode(0xbe,  7, this->cmp(CPURegister::RAM));
    execute_opcode(0xFE,  7, this->cpi(this->next_byte()));
    execute_opcode(0xC5, 11, this->push(CPURegister::BC));
    execute_opcode(0xD5, 11, this->push(CPURegister::DE));
    execute_opcode(0xE5, 11, this->push(CPURegister::HL));
    execute_opcode(0xF5, 11, this->push(CPURegister::PSW));
    execute_opcode(0xC1, 10, this->pop(CPURegister::BC));
    execute_opcode(0xD1, 10, this->pop(CPURegister::DE));
    execute_opcode(0xE1, 10, this->pop(CPURegister::HL));
    execute_opcode(0xF1, 10, this->pop(CPURegister::PSW));
    execute_opcode(0xC3, 10, this->jmp(this->next_address()));
    execute_opcode(0xCD, 17, this->call(this->next_address()));
    execute_opcode(0xC9, 11, this->ret());
    execute_opcode(0xC7, 10, this->rst(0x00));
    execute_opcode(0xCF, 10, this->rst(0x08));
    execute_opcode(0xD7, 10, this->rst(0x10));
    execute_opcode(0xDF, 10, this->rst(0x18));
    execute_opcode(0xE7, 10, this->rst(0x20));
    execute_opcode(0xEF, 10, this->rst(0x28));
    execute_opcode(0xF7, 10, this->rst(0x30));
    execute_opcode(0xFF, 10, this->rst(0x38));
    execute_opcode(0xCA, 10, this->jz(this->next_address()));
    execute_opcode(0xC2, 10, this->jnz(this->next_address()));
    execute_opcode(0xDA, 10, this->jc(this->next_address()));
    execute_opcode(0xD2, 10, this->jnc(this->next_address()));
    execute_opcode(0xEA, 10, this->jpe(this->next_address()));
    execute_opcode(0xE2, 10, this->jpo(this->next_address()));
    execute_opcode(0xFA, 10, this->jm(this->next_address()));
    execute_opcode(0xF2, 10, this->jp(this->next_address()));
    execute_opcode(0xCC, 14, this->cz(this->next_address()));
    execute_opcode(0xC4, 14, this->cnz(this->next_address()));
    execute_opcode(0xDC, 14, this->cc(this->next_address()));
    execute_opcode(0xD4, 14, this->cnc(this->next_address()));
    execute_opcode(0xEC, 14, this->cpe(this->next_address()));
    execute_opcode(0xE4, 14, this->cpo(this->next_address()));
    execute_opcode(0xFC, 14, this->cm(this->next_address()));
    execute_opcode(0xF4, 14, this->cp(this->next_address()));
    execute_opcode(0xC8,  8, this->rz());
    execute_opcode(0xC0,  8, this->rnz());
    execute_opcode(0xD8,  8, this->rc());
    execute_opcode(0xD0,  8, this->rnc());
    execute_opcode(0xE8,  8, this->rpe());
    execute_opcode(0xE0,  8, this->rpo());
    execute_opcode(0xF8,  8, this->rm());
    execute_opcode(0xF0,  8, this->rp());
    execute_opcode(0x3A, 13, this->lda(this->next_address()));
    execute_opcode(0x0A,  7, this->ldax(CPURegister::BC));
    execute_opcode(0x1A,  7, this->ldax(CPURegister::DE));
    execute_opcode(0x01, 10, this->lxi(CPURegister::BC, this->next_address()));
    execute_opcode(0x11, 10, this->lxi(CPURegister::DE, this->next_address()));
    execute_opcode(0x21, 10, this->lxi(CPURegister::HL, this->next_address()));
    execute_opcode(0x31, 10, this->lxi(CPURegister::StackPointer, this->next_address()));
    execute_opcode(0x2A, 16, this->lhld(this->next_address()));
    execute_opcode(0x32, 13, this->sta(this->next_address()));
    execute_opcode(0x02,  7, this->stax(CPURegister::BC));
    execute_opcode(0x12,  7, this->stax(CPURegister::DE));
    execute_opcode(0x22, 16, this->shld(this->next_address()));
    execute_opcode(0x09, 10, this->dad(CPURegister::BC));
    execute_opcode(0x19, 10, this->dad(CPURegister::DE));
    execute_opcode(0x29, 10, this->dad(CPURegister::HL));
    execute_opcode(0x39, 10, this->dad(CPURegister::StackPointer));
    execute_opcode(0x2F,  4, this->cma());
    execute_opcode(0x37,  4, this->stc());
    execute_opcode(0x3F,  4, this->cmc());
    execute_opcode(0xE9,  5, this->pchl());
    execute_opcode(0xF9,  5, this->sphl());
    execute_opcode(0xEB,  4, this->xchg());
    execute_opcode(0xE3, 18, this->xthl());
    execute_opcode(0xF3,  4, this->di());
    execute_opcode(0xFB,  4, this->ei());
    execute_opcode(0x27,  4, this->daa());
    execute_opcode(0xD3, 10, this->out(this->next_byte()));
    execute_opcode(0xDB, 10, this->in(this->next_byte()));
    execute_opcode(0x00,  4, {});
    #undef execute_opcode

    cout << "Opcode not implemented " << hex << static_cast<int>(read_opcode) << "!\n";
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
        case CPURegister::PSW: return address_from_high_low(this->A, this->flags.as_byte);
        case CPURegister::BC: return address_from_high_low(this->B, this->C);
        case CPURegister::DE: return address_from_high_low(this->D, this->E);
        case CPURegister::HL: return address_from_high_low(this->H, this->L);
        case CPURegister::StackPointer: return this->stack.get_pointer();
        case CPURegister::ProgramCounter: return this->program_counter;
        case CPURegister::RAM:
        {
            const u16 address = address_from_high_low(this->H, this->L);
            return (*this->ram)[address % RAM_SIZE];
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
        case CPURegister::PSW: this->A = value >> 8; this->flags.as_byte = value & 0xFF; break;
        case CPURegister::BC: this->B = value >> 8; this->C = value & 0xFF; break;
        case CPURegister::DE: this->D = value >> 8; this->E = value & 0xFF; break;
        case CPURegister::HL: this->H = value >> 8; this->L = value & 0xFF; break;
        case CPURegister::StackPointer: this->stack.set_pointer(value); break;
        case CPURegister::ProgramCounter: this->program_counter = value; break;
        case CPURegister::RAM:
        {
            const u16 address = address_from_high_low(this->H, this->L);
            (*this->ram)[address % RAM_SIZE] = value;
            break;
        }
    }
}

void CPU::update_arithmetic_flags(const u16 value)
{
    //update flags after arithmetic operation
    const u8 value_as_byte = value & 0xFF;
    this->flags.as_bits.zero = value_as_byte == 0;
    this->flags.as_bits.negative = value_as_byte > 0x7F;
    this->flags.as_bits.even = popcount(value_as_byte) % 2 == 0;
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = value > 0xFF;
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

void CPU::add(const CPURegister target)
{
    //add target to accumulator
    this->adi(this->read_register(target));
}

void CPU::adi(const u16 value)
{
    //add value to accumulator
    const u16 new_value = this->A + value;
    this->A = new_value;
    this->update_arithmetic_flags(new_value);
}

void CPU::adc(const CPURegister target)
{
    //add with carry target to accumulator
    this->aci(this->read_register(target));
}

void CPU::aci(const u16 value)
{
    //add with carry value to accumulator
    const u16 new_value = this->A + value + this->flags.as_bits.carry;
    this->A = new_value;
    this->update_arithmetic_flags(new_value);
}

void CPU::sub(const CPURegister target)
{
    //subtract target from accumulator
    this->sui(this->read_register(target));
}

void CPU::sui(const u16 value)
{
    //subtract value from accumulator
    const i16 new_value = this->A - value;
    this->A = new_value;
    this->update_arithmetic_flags(new_value);
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = new_value < 0 || new_value > 0xFF;
}

void CPU::sbb(const CPURegister target)
{
    //subtract target with carry from accumulator
    this->sbi(this->read_register(target));
}

void CPU::sbi(const u16 value)
{
    //subtract value with carry from accumulator
    const i16 new_value = this->A - value - this->flags.as_bits.carry;
    this->A = new_value;
    this->update_arithmetic_flags(new_value);
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = new_value < 0 || new_value > 0xFF;
}

void CPU::cmp(const CPURegister target)
{
    //compare target with accumulator
    this->cpi(this->read_register(target));
}

void CPU::cpi(const u16 value)
{
    //compare value with accumulator
    const i16 new_value = this->A - value;
    this->update_arithmetic_flags(new_value);
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = new_value < 0 || new_value > 0xFF;
}

void CPU::push(const CPURegister from)
{
    //push the stack
    const u16 value = this->read_register(from);
    this->stack.push_address(this, value);
}

void CPU::pop(const CPURegister to)
{
    //pop the stack
    const u16 value = this->stack.pop_address(this);
    this->write_register(to, value);
}

void CPU::jmp(const u16 address)
{
    //goto / jump
    this->program_counter = address;
}

void CPU::call(const u16 address)
{
    //jump to subroutine
    this->stack.push_address(this, this->program_counter);
    this->program_counter = address;
}

void CPU::ret()
{
    //return from subroutine
    this->program_counter = this->stack.pop_address(this);
}

void CPU::rst(const u16 address)
{
    //call subroutine at address
    this->call(address);
}

void CPU::jz(const u16 address)
{
    //jump if zero
    if (this->flags.as_bits.zero)
        this->jmp(address);
}

void CPU::cz(const u16 address)
{
    //jump to subroutine if zero
    if (this->flags.as_bits.zero)
        this->call(address);
}

void CPU::rz()
{
    //return from subroutine if zero
    if (this->flags.as_bits.zero)
        this->ret();
}

void CPU::jnz(const u16 address)
{
    //jump if not zero
    if (!this->flags.as_bits.zero)
        this->jmp(address);
}

void CPU::cnz(const u16 address)
{
    //jump to subroutine if not zero
    if (!this->flags.as_bits.zero)
        this->call(address);
}

void CPU::rnz()
{
    //return from subroutine if not zero
    if (!this->flags.as_bits.zero)
        this->ret();
}

void CPU::jc(const u16 address)
{
    //jump if carry
    if (this->flags.as_bits.carry)
        this->jmp(address);
}

void CPU::cc(const u16 address)
{
    //jump to subroutine if carry
    if (this->flags.as_bits.carry)
        this->call(address);
}

void CPU::rc()
{
    //return from subroutine if carry
    if (this->flags.as_bits.carry)
        this->ret();
}

void CPU::jnc(const u16 address)
{
    //jump if not carry
    if (!this->flags.as_bits.carry)
        this->jmp(address);
}

void CPU::cnc(const u16 address)
{
    //jump to subroutine if not carry
    if (!this->flags.as_bits.carry)
        this->call(address);
}

void CPU::rnc()
{
    //return from subroutine if not carry
    if (!this->flags.as_bits.carry)
        this->ret();
}

void CPU::jpe(const u16 address)
{
    //jump if even
    if (this->flags.as_bits.even)
        this->jmp(address);
}

void CPU::cpe(const u16 address)
{
    //jump to subroutine if even
    if (this->flags.as_bits.even)
        this->call(address);
}

void CPU::rpe()
{
    //return from subroutine if even
    if (this->flags.as_bits.even)
        this->ret();
}

void CPU::jpo(const u16 address)
{
    //jump if odd
    if (!this->flags.as_bits.even)
        this->jmp(address);
}

void CPU::cpo(const u16 address)
{
    //jump to subroutine if odd
    if (!this->flags.as_bits.even)
        this->call(address);
}

void CPU::rpo()
{
    //return from subroutine if odd
    if (!this->flags.as_bits.even)
        this->ret();
}

void CPU::jm(const u16 address)
{
    //jump if negative
    if (this->flags.as_bits.negative)
        this->jmp(address);
}

void CPU::cm(const u16 address)
{
    //jump to subroutine if negative
    if (this->flags.as_bits.negative)
        this->call(address);
}

void CPU::rm()
{
    //return from subroutine if negative
    if (this->flags.as_bits.negative)
        this->ret();
}

void CPU::jp(const u16 address)
{
    //jump if positive
    if (!this->flags.as_bits.negative)
        this->jmp(address);
}

void CPU::cp(const u16 address)
{
    //jump to subroutine if positive
    if (!this->flags.as_bits.negative)
        this->call(address);
}

void CPU::rp()
{
    //return from subroutine if positive
    if (!this->flags.as_bits.negative)
        this->ret();
}

void CPU::lda(const u16 address)
{
    //load accumulator with value from RAM at address
    this->A = (*this->ram)[address % RAM_SIZE];
}

void CPU::ldax(const CPURegister from)
{
    //load accumulator with value from register
    const u16 address = this->read_register(from);
    this->A = (*this->ram)[address % RAM_SIZE];
}

void CPU::lxi(const CPURegister to, const u16 value)
{
    //load register with value
    this->write_register(to, value);
}

void CPU::lhld(const u16 address)
{
    //load L,H registers with values from RAM at address
    this->L = (*this->ram)[address % RAM_SIZE];
    this->H = (*this->ram)[(address + 1) % RAM_SIZE];
}

void CPU::sta(const u16 address) const
{
    //store accumulator into RAM at address
    (*this->ram)[address % RAM_SIZE] = this->A;
}

void CPU::stax(const CPURegister to)
{
    //store accumulator into register
    this->write_register(to, this->A);
}

void CPU::shld(const u16 address) const
{
    //store L,H registers into RAM at address
    (*this->ram)[address % RAM_SIZE] = this->L;
    (*this->ram)[(address + 1) % RAM_SIZE] = this->H;
}

void CPU::dad(const CPURegister target)
{
    //add target to HL
    const u32 old_target_value = this->read_register(target);
    const u32 old_hl_value = this->read_register(CPURegister::HL);
    const u32 new_hl_value = old_hl_value + old_target_value;
    this->write_register(CPURegister::HL, new_hl_value);
    this->update_arithmetic_flags(new_hl_value);
    this->flags.as_bits.carry = this->flags.as_bits.aux_carry = new_hl_value > 0xFFFF;
}

void CPU::cma()
{
    //bitwise negate accumulator
    this->A = ~this->A;
}

void CPU::stc()
{
    //set carry flag
    this->flags.as_bits.carry = true;
}

void CPU::cmc()
{
    //toggle carry flag
    this->flags.as_bits.carry = !this->flags.as_bits.carry;
}

void CPU::pchl()
{
    //copy H,L into program counter
    const u16 value = this->read_register(CPURegister::HL);
    this->write_register(CPURegister::ProgramCounter, value);
}

void CPU::sphl()
{
    //copy H,L into stack pointer
    const u16 value = this->read_register(CPURegister::HL);
    this->write_register(CPURegister::StackPointer, value);
}

void CPU::xchg()
{
    //exchange HL with DE
    const u16 hl = this->read_register(CPURegister::HL);
    const u16 de = this->read_register(CPURegister::DE);
    this->write_register(CPURegister::HL, de);
    this->write_register(CPURegister::DE, hl);
}

void CPU::xthl()
{
    //exchange HL with value at RAM address stack pointer
    const u16 stack_pointer = this->stack.get_pointer();
    const u8 stack_first_value = (*this->ram)[stack_pointer % RAM_SIZE];
    const u8 stack_second_value = (*this->ram)[(stack_pointer + 1) % RAM_SIZE];
    (*this->ram)[stack_pointer % RAM_SIZE] = this->L;
    (*this->ram)[(stack_pointer + 1) % RAM_SIZE] = this->H;
    this->L = stack_first_value;
    this->H = stack_second_value;
}

void CPU::di()
{
    //disable interrupts
    this->are_interrupts_enabled = false;
}

void CPU::ei()
{
    //enable interrupts
    this->are_interrupts_enabled = true;
}

void CPU::daa()
{
    const u8 lsb = this->A & 0x0F;
    u8 msb = (this->A & 0xF0) >> 4;
    if (lsb > 9 || this->flags.as_bits.aux_carry)
    {
        this->A += 6;
        this->flags.as_bits.aux_carry = (lsb + 6) > 0x0F;
    }

    if (msb > 9 || this->flags.as_bits.carry) { msb += 6; }
    this->A = (msb << 4) | (this->A & 0x0F);
    this->flags.as_bits.aux_carry = (msb + 6) > 0x0F;
    this->update_arithmetic_flags(this->A);
}

void CPU::in(const u8 command)
{
    if (command == 1)
    {
        //todo implement in1: Player 1 keyboard
    }
    else if (command == 2)
    {
        //todo implement in2: Player 2 keyboard
    }
    else if (command == 3)
    {
        const u8 shift_amount = 8 - this->shift_register_offset;
        this->A = this->shift_register >> shift_amount;
    }
}

void CPU::out(const u8 command)
{
    if (command == 2)
    {
        this->shift_register_offset = this->A & 0x07;
    }
    else if (command == 4)
    {
        const u16 left = static_cast<u16>(this->A) << 8;
        const u16 right = this->shift_register >> 8;
        this->shift_register = left | right;
    }
}

void CPU::interrupt(const u16 number)
{
    if (this->are_interrupts_enabled)
    {
        this->are_interrupts_enabled = false;
        this->stack.push_address(this, this->program_counter);

        const u16 interrupt_vector = number << 3;
        this->program_counter = interrupt_vector;
    }
}
