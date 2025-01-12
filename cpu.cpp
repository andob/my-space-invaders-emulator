#include "cpu.h"
#include <iostream>

CPU::CPU(array<u8, RAM_SIZE>& ram) : ram(ram), stack(CPUStack(ram))
{
    //initialise fields
    this->A = this->B = this->C = this->D = this->E = 0;
    this->H = this->L = this->program_counter = 0;
    this->flags.as_byte = 0;
    this->shift_register = 0;
    this->shift_register_offset = 0;
    this->are_interrupts_enabled = true;
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

u16 CPU::next_address()
{
    const u8 low = this->next_byte();
    const u8 high = this->next_byte();
    return high << 8 | low;
}

void CPU::cpu_tick()
{
    //https://grantmestrength.github.io/RetroComputerInstructionManual/intel8080.html
    //todo check opcode hex codes
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
    else if (opcode == 0x21) this->inx(CPURegister::StackPointer);
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
    else if (opcode == 0x3B) this->dcx(CPURegister::StackPointer);
    else if (opcode == 0x57) this->add(CPURegister::A);
    else if (opcode == 0x50) this->add(CPURegister::B);
    else if (opcode == 0x51) this->add(CPURegister::C);
    else if (opcode == 0x52) this->add(CPURegister::D);
    else if (opcode == 0x53) this->add(CPURegister::E);
    else if (opcode == 0x54) this->add(CPURegister::H);
    else if (opcode == 0x55) this->add(CPURegister::L);
    else if (opcode == 0x56) this->add(CPURegister::RAM);
    else if (opcode == 0xC6) this->adi(this->next_byte());
    else if (opcode == 0x8F) this->adc(CPURegister::A);
    else if (opcode == 0x58) this->adc(CPURegister::B);
    else if (opcode == 0x59) this->adc(CPURegister::C);
    else if (opcode == 0x8A) this->adc(CPURegister::D);
    else if (opcode == 0x8B) this->adc(CPURegister::E);
    else if (opcode == 0x8C) this->adc(CPURegister::H);
    else if (opcode == 0x8D) this->adc(CPURegister::L);
    else if (opcode == 0x8E) this->adc(CPURegister::RAM);
    else if (opcode == 0xCE) this->aci(this->next_byte());
    else if (opcode == 0x61) this->sub(CPURegister::A);
    else if (opcode == 0x5A) this->sub(CPURegister::B);
    else if (opcode == 0x5B) this->sub(CPURegister::C);
    else if (opcode == 0x5C) this->sub(CPURegister::D);
    else if (opcode == 0x5D) this->sub(CPURegister::E);
    else if (opcode == 0x5E) this->sub(CPURegister::H);
    else if (opcode == 0x5F) this->sub(CPURegister::L);
    else if (opcode == 0x60) this->sub(CPURegister::RAM);
    else if (opcode == 0xD6) this->sui(this->next_byte());
    else if (opcode == 0x9F) this->sbb(CPURegister::A);
    else if (opcode == 0x62) this->sbb(CPURegister::B);
    else if (opcode == 0x63) this->sbb(CPURegister::C);
    else if (opcode == 0x9A) this->sbb(CPURegister::D);
    else if (opcode == 0x9B) this->sbb(CPURegister::E);
    else if (opcode == 0x9C) this->sbb(CPURegister::H);
    else if (opcode == 0x9D) this->sbb(CPURegister::L);
    else if (opcode == 0x9E) this->sbb(CPURegister::RAM);
    else if (opcode == 0xDE) this->sbi(this->next_byte());
    else if (opcode == 0xBF) this->cmp(CPURegister::A);
    else if (opcode == 0xB8) this->cmp(CPURegister::B);
    else if (opcode == 0xB9) this->cmp(CPURegister::C);
    else if (opcode == 0xBA) this->cmp(CPURegister::D);
    else if (opcode == 0xBB) this->cmp(CPURegister::E);
    else if (opcode == 0xBC) this->cmp(CPURegister::H);
    else if (opcode == 0xBD) this->cmp(CPURegister::L);
    else if (opcode == 0xBE) this->cmp(CPURegister::RAM);
    else if (opcode == 0xFE) this->cpi(this->next_byte());
    else if (opcode == 0xC5) this->push(CPURegister::BC);
    else if (opcode == 0xD5) this->push(CPURegister::DE);
    else if (opcode == 0xE5) this->push(CPURegister::HL);
    else if (opcode == 0xF5) this->push(CPURegister::PSW);
    else if (opcode == 0xC1) this->pop(CPURegister::BC);
    else if (opcode == 0xD1) this->pop(CPURegister::DE);
    else if (opcode == 0xE1) this->pop(CPURegister::HL);
    else if (opcode == 0xF1) this->pop(CPURegister::PSW);
    else if (opcode == 0xC3) this->jmp(this->next_address());
    else if (opcode == 0xCD) this->call(this->next_address());
    else if (opcode == 0xC9) this->ret();
    else if (opcode == 0xC7) this->rst(0x00);
    else if (opcode == 0xCF) this->rst(0x08);
    else if (opcode == 0xD7) this->rst(0x10);
    else if (opcode == 0xDF) this->rst(0x18);
    else if (opcode == 0xE7) this->rst(0x20);
    else if (opcode == 0xEF) this->rst(0x28);
    else if (opcode == 0xF7) this->rst(0x30);
    else if (opcode == 0xFF) this->rst(0x38);
    else if (opcode == 0xCA) this->jz(this->next_address());
    else if (opcode == 0xC2) this->jnz(this->next_address());
    else if (opcode == 0xDA) this->jc(this->next_address());
    else if (opcode == 0xD2) this->jnc(this->next_address());
    else if (opcode == 0xEA) this->jpe(this->next_address());
    else if (opcode == 0xE2) this->jpo(this->next_address());
    else if (opcode == 0xFA) this->jm(this->next_address());
    else if (opcode == 0xF2) this->jp(this->next_address());
    else if (opcode == 0xCC) this->cz(this->next_address());
    else if (opcode == 0xC4) this->cnz(this->next_address());
    else if (opcode == 0xDC) this->cc(this->next_address());
    else if (opcode == 0xD4) this->cnc(this->next_address());
    else if (opcode == 0xEC) this->cpe(this->next_address());
    else if (opcode == 0xE4) this->cpo(this->next_address());
    else if (opcode == 0xFC) this->cm(this->next_address());
    else if (opcode == 0xF4) this->cp(this->next_address());
    else if (opcode == 0xC8) this->rz();
    else if (opcode == 0xC0) this->rnz();
    else if (opcode == 0xD8) this->rc();
    else if (opcode == 0xD0) this->rnc();
    else if (opcode == 0xE8) this->rpe();
    else if (opcode == 0xE0) this->rpo();
    else if (opcode == 0xF8) this->rm();
    else if (opcode == 0xF0) this->rp();
    else if (opcode == 0x3A) this->lda(this->next_address());
    else if (opcode == 0x0A) this->ldax(CPURegister::BC);
    else if (opcode == 0x1A) this->ldax(CPURegister::DE);
    else if (opcode == 0x01) this->lxi(CPURegister::BC, this->next_address());
    else if (opcode == 0x0B) this->lxi(CPURegister::DE, this->next_address());
    else if (opcode == 0x15) this->lxi(CPURegister::HL, this->next_address());
    else if (opcode == 0x1F) this->lxi(CPURegister::StackPointer, this->next_address());
    else if (opcode == 0x2A) this->lhld(this->next_address());
    else if (opcode == 0x20) this->sta(this->next_address());
    else if (opcode == 0x02) this->stax(CPURegister::BC);
    else if (opcode == 0x0C) this->stax(CPURegister::DE);
    else if (opcode == 0x16) this->shld(this->next_address());
    else if (opcode == 0x09) this->dad(CPURegister::BC);
    else if (opcode == 0x13) this->dad(CPURegister::DE);
    else if (opcode == 0x1D) this->dad(CPURegister::HL);
    else if (opcode == 0x27) this->dad(CPURegister::StackPointer);
    else if (opcode == 0x2F) this->cma();
    else if (opcode == 0x25) this->stc();
    else if (opcode == 0x3F) this->cmc();
    else if (opcode == 0xE9) this->pchl();
    else if (opcode == 0xF9) this->sphl();
    else if (opcode == 0xEB) this->xchg();
    else if (opcode == 0xE3) this->xthl();
    else if (opcode == 0xF3) this->di();
    else if (opcode == 0xFB) this->ei();
    else if (opcode == 0x1B) this->daa();
    else if (opcode == 0xD3) this->out(this->next_byte());
    else if (opcode == 0xDB) this->in(this->next_byte());
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
        case CPURegister::PSW: return this->A << 8 | this->flags.as_byte;
        case CPURegister::BC: return this->B << 8 | this->C;
        case CPURegister::DE: return this->D << 8 | this->E;
        case CPURegister::HL: return this->H << 8 | this->L;
        case CPURegister::StackPointer: return this->stack.get_pointer();
        case CPURegister::ProgramCounter: return this->program_counter;
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
        case CPURegister::PSW: this->A = value >> 8; this->flags.as_byte = value & 0xFF; break;
        case CPURegister::BC: this->B = value >> 8; this->C = value & 0xFF; break;
        case CPURegister::DE: this->D = value >> 8; this->E = value & 0xFF; break;
        case CPURegister::HL: this->H = value >> 8; this->L = value & 0xFF; break;
        case CPURegister::StackPointer: this->stack.set_pointer(value); break;
        case CPURegister::ProgramCounter: this->program_counter = value; break;
        case CPURegister::RAM:
        {
            const u16 address = this->H << 8 | this->L;
            this->ram[address % RAM_SIZE] = value;
            break;
        }
    }
}

void CPU::update_arithmetic_flags(const u16 value)
{
    //update flags after arithmetic operation
    this->flags.as_bits.zero = (value & 0xFF) == 0;
    this->flags.as_bits.negative = (value & 0xFF) > 0x7F;
    this->flags.as_bits.even = (value & 0xFF) % 2 == 0;
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
    this->stack.push_address(value);
}

void CPU::pop(const CPURegister to)
{
    //pop the stack
    const u16 value = this->stack.pop_address();
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
    this->stack.push_address(this->program_counter);
    this->program_counter = address;
}

void CPU::ret()
{
    //return from subroutine
    this->program_counter = this->stack.pop_address();
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
    this->A = this->ram[address % RAM_SIZE];
}

void CPU::ldax(const CPURegister from)
{
    //load accumulator with value from register
    this->A = this->read_register(from);
}

void CPU::lxi(const CPURegister to, const u16 address)
{
    //load register with value from RAM at address
    const u16 value = this->ram[address % RAM_SIZE];
    this->write_register(to, value);
}

void CPU::lhld(const u16 address)
{
    //load L,H registers with values from RAM at address
    this->L = this->ram[address % RAM_SIZE];
    this->H = this->ram[(address + 1) % RAM_SIZE];
}

void CPU::sta(const u16 address) const
{
    //store accumulator into RAM at address
    this->ram[address % RAM_SIZE] = this->A;
}

void CPU::stax(const CPURegister to)
{
    //store accumulator into register
    this->write_register(to, this->A);
}

void CPU::shld(const u16 address) const
{
    //store L,H registers into RAM at address
    this->ram[address % RAM_SIZE] = this->L;
    this->ram[(address + 1) % RAM_SIZE] = this->H;
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
    const u8 stack_first_value = this->ram[stack_pointer % RAM_SIZE];
    const u8 stack_second_value = this->ram[(stack_pointer + 1) % RAM_SIZE];
    this->ram[stack_pointer % RAM_SIZE] = this->L;
    this->ram[(stack_pointer + 1) % RAM_SIZE] = this->H;
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

void CPU::run()
{
    this->cpu_tick();
}
