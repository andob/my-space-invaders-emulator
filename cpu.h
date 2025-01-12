#ifndef CPU_H
#define CPU_H

#include "constants.h"
#include <array>
#include <vector>

using namespace std;

union CPUFlags
{
    struct
    {
        bool negative : 1;
        bool zero : 1;
        bool reserved1 : 1;
        bool aux_carry : 1;
        bool reserved2 : 1;
        bool even : 1;
        bool reserved3 : 1;
        bool carry : 1;
    } as_bits;
    u8 as_byte;
};

enum CPURegister
{cpu
    A, B, C, D, E, H, L,
    PSW, BC, DE, HL, SP, RAM
};

class CPUStack
{
public:
    u16 pointer;
    void push_byte(array<u8, RAM_SIZE>& ram, u8 value);
    void push_address(array<u8, RAM_SIZE>& ram, u16 address);
    u8 pop_byte(const array<u8, RAM_SIZE>& ram);
    u16 pop_address(const array<u8, RAM_SIZE>& ram);
};

class CPU
{
    array<u8, RAM_SIZE> ram;
    u8 A, B, C, D, E, H, L;
    u16 program_counter;
    CPUStack stack;
    CPUFlags flags;

    u8 next_byte();
    u16 next_address();

    void cpu_tick();

    u16 read_register(CPURegister register);
    void write_register(CPURegister register, u16 value);

    void mov(CPURegister from, CPURegister to);
    void mvi(u16 value, CPURegister to);
    void ana(CPURegister target);
    void xra(CPURegister target);
    void ora(CPURegister target);
    void ani(u16 value);
    void xri(u16 value);
    void ori(u16 value);
    void rrc();
    void rar();
    void rlc();
    void ral();
    void inr(CPURegister target);
    void inx(CPURegister target);
    void dcr(CPURegister target);
    void dcx(CPURegister target);
    void add(CPURegister target);
    void adi(u16 value);
    void adc(CPURegister target);
    void aci(u16 value);
    void sub(CPURegister target);
    void sui(u16 value);
    void sbb(CPURegister target);
    void sbi(u16 value);
    void cmp(CPURegister target);
    void cpi(u16 value);
    void push(CPURegister from);
    void pop(CPURegister to);
    void jmp(u16 address);
    void call(u16 address);
    void ret();
    void rst(u16 address);
    void jz(u16 address);
    void cz(u16 address);
    void rz();
    void jnz(u16 address);
    void cnz(u16 address);
    void rnz();
    void jc(u16 address);
    void cc(u16 address);
    void rc();
    void jnc(u16 address);
    void cnc(u16 address);
    void rnc();
    void jpe(u16 address);
    void cpe(u16 address);
    void rpe();
    void jpo(u16 address);
    void cpo(u16 address);
    void rpo();
    void jm(u16 address);
    void cm(u16 address);
    void rm();
    void jp(u16 address);
    void cp(u16 address);
    void rp();
    void lda(u16 address);
    void ldax(CPURegister from);
    void lxi(CPURegister to, u16 address);
    void lhld(u16 address);
    void sta(u16 address);
    void stax(CPURegister to);
    void shld(u16 address);

    void update_arithmetic_flags(u16 value);

public:
    explicit CPU(const vector<u8>& rom_bytes);

    void run();
};

#endif //CPU_H
