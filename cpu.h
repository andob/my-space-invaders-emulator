#ifndef CPU_H
#define CPU_H

#include "constants.h"
#include "cpu/flags.h"
#include "cpu/registers.h"
#include "cpu/stack.h"
#include <array>
#include <vector>

using namespace std;

class CPU
{
    array<u8, RAM_SIZE>& ram;
    u8 A, B, C, D, E, H, L;
    u16 program_counter;
    CPUStack stack;
    CPUFlags flags;
    u16 shift_register;
    u8 shift_register_offset;
    //todo implement interrupts
    bool are_interrupts_enabled;

    u8 next_byte();
    u16 next_address();

    void cpu_tick();

    u16 read_register(CPURegister register);
    void write_register(CPURegister register, u16 value);

    void update_arithmetic_flags(u16 value);

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
    void sta(u16 address) const;
    void stax(CPURegister to);
    void shld(u16 address) const;
    void dad(CPURegister target);
    void cma();
    void stc();
    void cmc();
    void pchl();
    void sphl();
    void xchg();
    void xthl();
    void di();
    void ei();
    void daa();
    void in(u8 command);
    void out(u8 command);

public:
    explicit CPU(array<u8, RAM_SIZE>& ram);

    void run();
};

#endif //CPU_H
