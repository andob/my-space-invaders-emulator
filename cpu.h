#ifndef CPU_H
#define CPU_H

#include "constants.h"
#include <array>
#include <stack>
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
{
    A, B, C, D, E, H, L,
    BC, DE, HL, SP, RAM
};

class CPU
{
    array<u8, RAM_SIZE> ram;
    u8 A, B, C, D, E, H, L;
    u16 program_counter;
    u16 stack_pointer;
    CPUFlags flags;

    u8 next_byte();
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

    void update_arithmetic_flags(u16 value);

public:
    explicit CPU(const vector<u8>& rom_bytes);

    void run();
};

#endif //CPU_H
