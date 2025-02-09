use crate::system::cpu::CPU;
use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::registers::CPURegister;
use crate::system::cpu::stack::CPUStack;

type OpcodeFn = dyn Fn(&mut CPU) -> ();
type OpcodeFnWithArg = dyn Fn(&mut CPU, u16) -> ();

pub struct Opcode
{
    pub key : u8,
    pub name : String,
    pub lambda : Box<OpcodeFn>,
    pub duration : u8,
}

macro_rules! opcode
{
    ($key : expr, $name : expr, $duration : expr) =>
    {
        Opcode
        {
            key: $key, lambda: $name,
            name: stringify!($name).to_string(),
            duration: $duration,
        }
    }
}

pub fn build_opcodes_slice() -> Box<[Opcode]>
{
    let mut opcodes = vec!
    [
        opcode!(0x00, nop(), 4),
        opcode!(0x08, nop(), 4),
        opcode!(0x10, nop(), 4),
        opcode!(0x18, nop(), 4),
        opcode!(0x20, nop(), 4),
        opcode!(0x28, nop(), 4),
        opcode!(0x30, nop(), 4),
        opcode!(0x38, nop(), 4),
        opcode!(0x76, nop(), 4),
        opcode!(0xCB, nop(), 4),
        opcode!(0xD9, nop(), 4),
        opcode!(0xDD, nop(), 4),
        opcode!(0xED, nop(), 4),
        opcode!(0xFD, nop(), 4),
        opcode!(0x7F, mov(CPURegister::A, CPURegister::A), 5),
        opcode!(0x47, mov(CPURegister::A, CPURegister::B), 5),
        opcode!(0x4f, mov(CPURegister::A, CPURegister::C), 5),
        opcode!(0x57, mov(CPURegister::A, CPURegister::D), 5),
        opcode!(0x5f, mov(CPURegister::A, CPURegister::E), 5),
        opcode!(0x67, mov(CPURegister::A, CPURegister::H), 5),
        opcode!(0x6f, mov(CPURegister::A, CPURegister::L), 5),
        opcode!(0x77, mov(CPURegister::A, CPURegister::RAM), 7),
        opcode!(0x78, mov(CPURegister::B, CPURegister::A), 5),
        opcode!(0x40, mov(CPURegister::B, CPURegister::B), 5),
        opcode!(0x48, mov(CPURegister::B, CPURegister::C), 5),
        opcode!(0x50, mov(CPURegister::B, CPURegister::D), 5),
        opcode!(0x58, mov(CPURegister::B, CPURegister::E), 5),
        opcode!(0x60, mov(CPURegister::B, CPURegister::H), 5),
        opcode!(0x68, mov(CPURegister::B, CPURegister::L), 5),
        opcode!(0x70, mov(CPURegister::B, CPURegister::RAM), 7),
        opcode!(0x79, mov(CPURegister::C, CPURegister::A), 5),
        opcode!(0x41, mov(CPURegister::C, CPURegister::B), 5),
        opcode!(0x49, mov(CPURegister::C, CPURegister::C), 5),
        opcode!(0x51, mov(CPURegister::C, CPURegister::D), 5),
        opcode!(0x59, mov(CPURegister::C, CPURegister::E), 5),
        opcode!(0x61, mov(CPURegister::C, CPURegister::H), 5),
        opcode!(0x69, mov(CPURegister::C, CPURegister::L), 5),
        opcode!(0x71, mov(CPURegister::C, CPURegister::RAM), 7),
        opcode!(0x7a, mov(CPURegister::D, CPURegister::A), 5),
        opcode!(0x42, mov(CPURegister::D, CPURegister::B), 5),
        opcode!(0x4a, mov(CPURegister::D, CPURegister::C), 5),
        opcode!(0x52, mov(CPURegister::D, CPURegister::D), 5),
        opcode!(0x5a, mov(CPURegister::D, CPURegister::E), 5),
        opcode!(0x62, mov(CPURegister::D, CPURegister::H), 5),
        opcode!(0x6a, mov(CPURegister::D, CPURegister::L), 5),
        opcode!(0x72, mov(CPURegister::D, CPURegister::RAM), 7),
        opcode!(0x7b, mov(CPURegister::E, CPURegister::A), 5),
        opcode!(0x43, mov(CPURegister::E, CPURegister::B), 5),
        opcode!(0x4b, mov(CPURegister::E, CPURegister::C), 5),
        opcode!(0x53, mov(CPURegister::E, CPURegister::D), 5),
        opcode!(0x5b, mov(CPURegister::E, CPURegister::E), 5),
        opcode!(0x63, mov(CPURegister::E, CPURegister::H), 5),
        opcode!(0x6b, mov(CPURegister::E, CPURegister::L), 5),
        opcode!(0x73, mov(CPURegister::E, CPURegister::RAM), 7),
        opcode!(0x7c, mov(CPURegister::H, CPURegister::A), 5),
        opcode!(0x44, mov(CPURegister::H, CPURegister::B), 5),
        opcode!(0x4c, mov(CPURegister::H, CPURegister::C), 5),
        opcode!(0x54, mov(CPURegister::H, CPURegister::D), 5),
        opcode!(0x5c, mov(CPURegister::H, CPURegister::E), 5),
        opcode!(0x64, mov(CPURegister::H, CPURegister::H), 5),
        opcode!(0x6c, mov(CPURegister::H, CPURegister::L), 5),
        opcode!(0x74, mov(CPURegister::H, CPURegister::RAM), 7),
        opcode!(0x7d, mov(CPURegister::L, CPURegister::A), 5),
        opcode!(0x45, mov(CPURegister::L, CPURegister::B), 5),
        opcode!(0x4d, mov(CPURegister::L, CPURegister::C), 5),
        opcode!(0x55, mov(CPURegister::L, CPURegister::D), 5),
        opcode!(0x5d, mov(CPURegister::L, CPURegister::E), 5),
        opcode!(0x65, mov(CPURegister::L, CPURegister::H), 5),
        opcode!(0x6d, mov(CPURegister::L, CPURegister::L), 5),
        opcode!(0x75, mov(CPURegister::L, CPURegister::RAM), 7),
        opcode!(0x7e, mov(CPURegister::RAM, CPURegister::A), 7),
        opcode!(0x46, mov(CPURegister::RAM, CPURegister::B), 7),
        opcode!(0x4e, mov(CPURegister::RAM, CPURegister::C), 7),
        opcode!(0x56, mov(CPURegister::RAM, CPURegister::D), 7),
        opcode!(0x5e, mov(CPURegister::RAM, CPURegister::E), 7),
        opcode!(0x66, mov(CPURegister::RAM, CPURegister::H), 7),
        opcode!(0x6e, mov(CPURegister::RAM, CPURegister::L), 7),
        opcode!(0x3E, read_byte_then(mvi(CPURegister::A)), 7),
        opcode!(0x06, read_byte_then(mvi(CPURegister::B)), 7),
        opcode!(0x0E, read_byte_then(mvi(CPURegister::C)), 7),
        opcode!(0x16, read_byte_then(mvi(CPURegister::D)), 7),
        opcode!(0x1E, read_byte_then(mvi(CPURegister::E)), 7),
        opcode!(0x26, read_byte_then(mvi(CPURegister::H)), 7),
        opcode!(0x2E, read_byte_then(mvi(CPURegister::L)), 7),
        opcode!(0x36, read_byte_then(mvi(CPURegister::RAM)), 10),
        opcode!(0xa7, ana(CPURegister::A), 4),
        opcode!(0xa0, ana(CPURegister::B), 4),
        opcode!(0xa1, ana(CPURegister::C), 4),
        opcode!(0xa2, ana(CPURegister::D), 4),
        opcode!(0xa3, ana(CPURegister::E), 4),
        opcode!(0xa4, ana(CPURegister::H), 4),
        opcode!(0xa5, ana(CPURegister::L), 4),
        opcode!(0xa6, ana(CPURegister::RAM), 7),
        opcode!(0xE6, read_byte_then(ani()), 7),
        opcode!(0xaf, xra(CPURegister::A), 4),
        opcode!(0xa8, xra(CPURegister::B), 4),
        opcode!(0xa9, xra(CPURegister::C), 4),
        opcode!(0xaa, xra(CPURegister::D), 4),
        opcode!(0xab, xra(CPURegister::E), 4),
        opcode!(0xac, xra(CPURegister::H), 4),
        opcode!(0xad, xra(CPURegister::L), 4),
        opcode!(0xae, xra(CPURegister::RAM), 7),
        opcode!(0xEE, read_byte_then(xri()), 7),
        opcode!(0xb7, ora(CPURegister::A), 4),
        opcode!(0xb0, ora(CPURegister::B), 4),
        opcode!(0xb1, ora(CPURegister::C), 4),
        opcode!(0xb2, ora(CPURegister::D), 4),
        opcode!(0xb3, ora(CPURegister::E), 4),
        opcode!(0xb4, ora(CPURegister::H), 4),
        opcode!(0xb5, ora(CPURegister::L), 4),
        opcode!(0xb6, ora(CPURegister::RAM), 7),
        opcode!(0xF6, read_byte_then(ori()), 7),
        opcode!(0x0F, rrc(), 4),
        opcode!(0x1F, rar(), 4),
        opcode!(0x07, rlc(), 4),
        opcode!(0x17, ral(), 4),
        opcode!(0x3C, inr(CPURegister::A), 5),
        opcode!(0x04, inr(CPURegister::B), 5),
        opcode!(0x0C, inr(CPURegister::C), 5),
        opcode!(0x14, inr(CPURegister::D), 5),
        opcode!(0x1C, inr(CPURegister::E), 5),
        opcode!(0x24, inr(CPURegister::H), 5),
        opcode!(0x2C, inr(CPURegister::L), 5),
        opcode!(0x34, inr(CPURegister::RAM), 10),
        opcode!(0x03, inx(CPURegister::BC), 5),
        opcode!(0x13, inx(CPURegister::DE), 5),
        opcode!(0x23, inx(CPURegister::HL), 5),
        opcode!(0x33, inx(CPURegister::StackPointer), 5),
        opcode!(0x3D, dcr(CPURegister::A), 5),
        opcode!(0x05, dcr(CPURegister::B), 5),
        opcode!(0x0D, dcr(CPURegister::C), 5),
        opcode!(0x15, dcr(CPURegister::D), 5),
        opcode!(0x1D, dcr(CPURegister::E), 5),
        opcode!(0x25, dcr(CPURegister::H), 5),
        opcode!(0x2D, dcr(CPURegister::L), 5),
        opcode!(0x35, dcr(CPURegister::RAM), 10),
        opcode!(0x0B, dcx(CPURegister::BC), 5),
        opcode!(0x1B, dcx(CPURegister::DE), 5),
        opcode!(0x2B, dcx(CPURegister::HL), 5),
        opcode!(0x3B, dcx(CPURegister::StackPointer), 5),
        opcode!(0x87, add(CPURegister::A), 4),
        opcode!(0x80, add(CPURegister::B), 4),
        opcode!(0x81, add(CPURegister::C), 4),
        opcode!(0x82, add(CPURegister::D), 4),
        opcode!(0x83, add(CPURegister::E), 4),
        opcode!(0x84, add(CPURegister::H), 4),
        opcode!(0x85, add(CPURegister::L), 4),
        opcode!(0x86, add(CPURegister::RAM), 7),
        opcode!(0xC6, read_byte_then(adi()), 7),
        opcode!(0x8f, adc(CPURegister::A), 4),
        opcode!(0x88, adc(CPURegister::B), 4),
        opcode!(0x89, adc(CPURegister::C), 4),
        opcode!(0x8a, adc(CPURegister::D), 4),
        opcode!(0x8b, adc(CPURegister::E), 4),
        opcode!(0x8c, adc(CPURegister::H), 4),
        opcode!(0x8d, adc(CPURegister::L), 4),
        opcode!(0x8e, adc(CPURegister::RAM), 7),
        opcode!(0xCE, read_byte_then(aci()), 7),
        opcode!(0x97, sub(CPURegister::A), 4),
        opcode!(0x90, sub(CPURegister::B), 4),
        opcode!(0x91, sub(CPURegister::C), 4),
        opcode!(0x92, sub(CPURegister::D), 4),
        opcode!(0x93, sub(CPURegister::E), 4),
        opcode!(0x94, sub(CPURegister::H), 4),
        opcode!(0x95, sub(CPURegister::L), 4),
        opcode!(0x96, sub(CPURegister::RAM), 7),
        opcode!(0xD6, read_byte_then(sui()), 7),
        opcode!(0x9f, sbb(CPURegister::A), 4),
        opcode!(0x98, sbb(CPURegister::B), 4),
        opcode!(0x99, sbb(CPURegister::C), 4),
        opcode!(0x9a, sbb(CPURegister::D), 4),
        opcode!(0x9b, sbb(CPURegister::E), 4),
        opcode!(0x9c, sbb(CPURegister::H), 4),
        opcode!(0x9d, sbb(CPURegister::L), 4),
        opcode!(0x9e, sbb(CPURegister::RAM), 7),
        opcode!(0xDE, read_byte_then(sbi()), 7),
        opcode!(0xbf, cmp(CPURegister::A), 4),
        opcode!(0xb8, cmp(CPURegister::B), 4),
        opcode!(0xb9, cmp(CPURegister::C), 4),
        opcode!(0xba, cmp(CPURegister::D), 4),
        opcode!(0xbb, cmp(CPURegister::E), 4),
        opcode!(0xbc, cmp(CPURegister::H), 4),
        opcode!(0xbd, cmp(CPURegister::L), 4),
        opcode!(0xbe, cmp(CPURegister::RAM), 7),
        opcode!(0xFE, read_byte_then(cpi()), 7),
        opcode!(0xC5, push(CPURegister::BC), 11),
        opcode!(0xD5, push(CPURegister::DE), 11),
        opcode!(0xE5, push(CPURegister::HL), 11),
        opcode!(0xF5, push(CPURegister::PSW), 11),
        opcode!(0xC1, pop(CPURegister::BC), 10),
        opcode!(0xD1, pop(CPURegister::DE), 10),
        opcode!(0xE1, pop(CPURegister::HL), 10),
        opcode!(0xF1, pop(CPURegister::PSW), 10),
        opcode!(0xC3, read_address_then(jmp()), 10),
        opcode!(0xCD, read_address_then(call()), 17),
        opcode!(0xC9, ret(), 11),
        opcode!(0xC7, rst(0x00), 10),
        opcode!(0xCF, rst(0x08), 10),
        opcode!(0xD7, rst(0x10), 10),
        opcode!(0xDF, rst(0x18), 10),
        opcode!(0xE7, rst(0x20), 10),
        opcode!(0xEF, rst(0x28), 10),
        opcode!(0xF7, rst(0x30), 10),
        opcode!(0xFF, rst(0x38), 10),
        opcode!(0xCA, read_address_then(jmp_if(|flags| flags.zero)), 10),
        opcode!(0xC2, read_address_then(jmp_if(|flags| !flags.zero)), 10),
        opcode!(0xDA, read_address_then(jmp_if(|flags| flags.carry)), 10),
        opcode!(0xD2, read_address_then(jmp_if(|flags| !flags.carry)), 10),
        opcode!(0xEA, read_address_then(jmp_if(|flags| flags.even)), 10),
        opcode!(0xE2, read_address_then(jmp_if(|flags| !flags.even)), 10),
        opcode!(0xFA, read_address_then(jmp_if(|flags| flags.negative)), 10),
        opcode!(0xF2, read_address_then(jmp_if(|flags| !flags.negative)), 10),
        opcode!(0xCC, read_address_then(call_if(|flags| flags.zero)), 14),
        opcode!(0xC4, read_address_then(call_if(|flags| !flags.zero)), 14),
        opcode!(0xDC, read_address_then(call_if(|flags| flags.carry)), 14),
        opcode!(0xD4, read_address_then(call_if(|flags| !flags.carry)), 14),
        opcode!(0xEC, read_address_then(call_if(|flags| flags.even)), 14),
        opcode!(0xE4, read_address_then(call_if(|flags| !flags.even)), 14),
        opcode!(0xFC, read_address_then(call_if(|flags| flags.negative)), 14),
        opcode!(0xF4, read_address_then(call_if(|flags| !flags.negative)), 14),
        opcode!(0xC8, ret_if(|flags| flags.zero), 8),
        opcode!(0xC0, ret_if(|flags| !flags.zero), 8),
        opcode!(0xD8, ret_if(|flags| flags.carry), 8),
        opcode!(0xD0, ret_if(|flags| !flags.carry), 8),
        opcode!(0xE8, ret_if(|flags| flags.even), 8),
        opcode!(0xE0, ret_if(|flags| !flags.even), 8),
        opcode!(0xF8, ret_if(|flags| flags.negative), 8),
        opcode!(0xF0, ret_if(|flags| !flags.negative), 8),
        opcode!(0x3A, read_address_then(lda()), 13),
        opcode!(0x0A, ldax(CPURegister::BC), 7),
        opcode!(0x1A, ldax(CPURegister::DE), 7),
        opcode!(0x01, read_address_then(lxi(CPURegister::BC)), 10),
        opcode!(0x11, read_address_then(lxi(CPURegister::DE)), 10),
        opcode!(0x21, read_address_then(lxi(CPURegister::HL)), 10),
        opcode!(0x31, read_address_then(lxi(CPURegister::StackPointer)), 10),
        opcode!(0x2A, read_address_then(lhld()), 16),
        opcode!(0x32, read_address_then(sta()), 13),
        opcode!(0x02, stax(CPURegister::BC), 7),
        opcode!(0x12, stax(CPURegister::DE), 7),
        opcode!(0x22, read_address_then(shld()), 16),
        opcode!(0x09, dad(CPURegister::BC), 10),
        opcode!(0x19, dad(CPURegister::DE), 10),
        opcode!(0x29, dad(CPURegister::HL), 10),
        opcode!(0x39, dad(CPURegister::StackPointer), 10),
        opcode!(0x2F, cma(), 4),
        opcode!(0x37, stc(), 4),
        opcode!(0x3F, cmc(), 4),
        opcode!(0xE9, pchl(), 5),
        opcode!(0xF9, sphl(), 5),
        opcode!(0xEB, xchg(), 4),
        opcode!(0xE3, xthl(), 18),
        opcode!(0xF3, di(), 4),
        opcode!(0xFB, ei(), 4),
        opcode!(0x27, daa(), 4),
        opcode!(0xD3, read_byte_then(out()), 10),
        opcode!(0xDB, read_byte_then(_in()), 10),
    ];

    (0x00..=0xFFu8).into_iter() //find unimplemented opcodes
        .filter(|key| opcodes.iter().find(|opcode| opcode.key==*key).is_none())
        .for_each(|key| { panic!("CPU Opcode {:#04X} is not implemented!", key) });

    (0x00..=0xFFu8).into_iter() //find duplicates
        .filter(|key| opcodes.iter().filter(|opcode| opcode.key==*key).count()>=2)
        .for_each(|key| { panic!("CPU Opcode {:#04X} is implemented twice!", key) });

    opcodes.sort_by(|o1, o2| u8::cmp(&o1.key, &o2.key));
    return opcodes.into_boxed_slice();
}

fn read_byte_then(lambda : Box<OpcodeFnWithArg>) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        let byte = cpu.next_byte();
        lambda(cpu, byte as u16);
    });
}

fn read_address_then(lambda : Box<OpcodeFnWithArg>) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        let address = cpu.next_address();
        lambda(cpu, address);
    })
}

fn update_arithmetic_flags(cpu : &mut CPU, value : u16)
{
    //update flags after arithmetic operation
    let value_as_byte = (value & 0xFF) as u8;
    cpu.flags.zero = value_as_byte == 0;
    cpu.flags.negative = value_as_byte > 0x7F;
    cpu.flags.even = value_as_byte.count_ones() % 2 == 0;
    cpu.flags.carry = value > 0xFF;
    cpu.flags.aux_carry = value > 0xFF;
}

fn nop() -> Box<OpcodeFn>
{
    return Box::new(|_| {});
}

fn mov(from : CPURegister, to : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //copy value of from register into to register
        let value = cpu.read_register(from);
        cpu.write_register(to, value);
    })
}

fn mvi(to : CPURegister) -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //copy value into register
        cpu.write_register(to, value as u16);
    })
}

fn ana(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //logical AND value with Accumulator
        let value = cpu.read_register(target) as u8;
        ani()(cpu, value as u16);
    })
}

fn ani() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //logical AND value with Accumulator
        let new_value = (cpu.A as u16) & value;
        cpu.A = new_value as u8;
        update_arithmetic_flags(cpu, new_value);
    })
}

fn xra(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //logical XOR value with Accumulator
        let value = cpu.read_register(target) as u8;
        xri()(cpu, value as u16);
    })
}

fn xri() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //logical XOR value with Accumulator
        let new_value = (cpu.A as u16) ^ value;
        cpu.A = new_value as u8;
        update_arithmetic_flags(cpu, new_value);
    })
}

fn ora(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //logical OR value with Accumulator
        let value = cpu.read_register(target) as u8;
        ori()(cpu, value as u16);
    })
}

fn ori() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //logical OR value with Accumulator
        let new_value = (cpu.A as u16) | value;
        cpu.A = new_value as u8;
        update_arithmetic_flags(cpu, new_value);
    })
}

fn rrc() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //shift right
        let bit_to_carry = cpu.A & 1;
        let bit_to_push = bit_to_carry;
        cpu.flags.carry = bit_to_carry == 1;
        cpu.A = (bit_to_push << 7) | (cpu.A >> 1);
    })
}

fn rar() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //shift right
        let bit_to_carry = cpu.A & 1;
        let bit_to_push = cpu.flags.carry as u8;
        cpu.flags.carry = bit_to_carry == 1;
        cpu.A = (bit_to_push << 7) | (cpu.A >> 1);
    })
}

fn rlc() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //shift left
        let bit_to_carry = cpu.A >> 7;
        let bit_to_push = bit_to_carry;
        cpu.flags.carry = bit_to_carry == 1;
        cpu.A = (cpu.A << 1) | bit_to_push;
    })
}

fn ral() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //shift left
        let bit_to_carry = cpu.A >> 7;
        let bit_to_push = cpu.flags.carry as u8;
        cpu.flags.carry = bit_to_carry == 1;
        cpu.A = (cpu.A << 1) | bit_to_push;
    })
}

fn inr(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //increment register value
        let old_value = cpu.read_register(target);
        let new_value = old_value + 1;
        cpu.write_register(target, new_value);
        update_arithmetic_flags(cpu, new_value);
        cpu.flags.aux_carry = old_value == 0xFF;
        cpu.flags.carry = cpu.flags.aux_carry;
    })
}

fn inx(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //increment register value
        let old_value = cpu.read_register(target);
        let new_value = old_value + 1;
        cpu.write_register(target, new_value);
        update_arithmetic_flags(cpu, new_value);
        cpu.flags.aux_carry = old_value == 0xFFFF;
        cpu.flags.carry = cpu.flags.aux_carry;
    })
}

fn dcr(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //decrement register value
        let old_value = cpu.read_register(target);
        let new_value = old_value - 1;
        cpu.write_register(target, new_value);
        update_arithmetic_flags(cpu, new_value);
        cpu.flags.aux_carry = old_value == 0;
        cpu.flags.carry = cpu.flags.aux_carry;
    })
}

fn dcx(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //decrement register value
        let old_value = cpu.read_register(target);
        let new_value = old_value - 1;
        cpu.write_register(target, new_value);
        update_arithmetic_flags(cpu, new_value);
        cpu.flags.aux_carry = old_value == 0;
        cpu.flags.carry = cpu.flags.aux_carry;
    })
}

fn add(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //add target to accumulator
        let value = cpu.read_register(target) as u8;
        adi()(cpu, value as u16);
    })
}

fn adi() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //add value to accumulator
        let new_value = (cpu.A as u16) + value;
        cpu.A = new_value as u8;
        update_arithmetic_flags(cpu, new_value);
    })
}

fn adc(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //add with carry target to accumulator
        let value = cpu.read_register(target) as u8;
        aci()(cpu, value as u16);
    })
}

fn aci() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //add with carry value to accumulator
        let new_value = (cpu.A as u16) + value + (cpu.flags.carry as u16);
        cpu.A = new_value as u8;
        update_arithmetic_flags(cpu, new_value);
    })
}

fn sub(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //subtract target from accumulator
        let value = cpu.read_register(target) as u8;
        sui()(cpu, value as u16);
    })
}

fn sui() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //subtract value from accumulator
        let new_value = (cpu.A as u16) - value;
        cpu.A = new_value as u8;
        update_arithmetic_flags(cpu, new_value);
        cpu.flags.aux_carry = new_value < 0 || new_value > 0xFF;
        cpu.flags.carry = cpu.flags.aux_carry;
    })
}

fn sbb(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //subtract target with carry from accumulator
        let value = cpu.read_register(target) as u8;
        sbi()(cpu, value as u16);
    })
}

fn sbi() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //subtract value with carry from accumulator
        let new_value = (cpu.A as u16) - value - (cpu.flags.carry as u16);
        cpu.A = new_value as u8;
        update_arithmetic_flags(cpu, new_value);
        cpu.flags.aux_carry = new_value < 0 || new_value > 0xFF;
        cpu.flags.carry = cpu.flags.aux_carry;
    })
}

fn cmp(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //compare target with accumulator
        let value = cpu.read_register(target) as u8;
        cpi()(cpu, value as u16);
    })
}

fn cpi() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //compare value with accumulator
        let new_value = (cpu.A as u16) - value;
        update_arithmetic_flags(cpu, new_value);
        cpu.flags.aux_carry = new_value < 0 || new_value > 0xFF;
        cpu.flags.carry = cpu.flags.aux_carry;
    })
}

fn push(from : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //push the stack
        let value = cpu.read_register(from);
        CPUStack::push_address(cpu, value);
    })
}

fn pop(to : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //pop the stack
        let value = CPUStack::pop_address(cpu);
        cpu.write_register(to, value);
    })
}

fn jmp() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, address|
    {
        //goto / jump
        cpu.program_counter = address;
    })
}

fn jmp_if(condition : fn(&CPUFlags) -> bool) -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, address|
    {
        if condition(&cpu.flags)
        {
            jmp()(cpu, address);
        }
    })
}

fn call() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, address|
    {
        //jump to subroutine
        CPUStack::push_address(cpu, cpu.program_counter);
        cpu.program_counter = address;
    })
}

fn call_if(condition : fn(&CPUFlags) -> bool) -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, address|
    {
        if condition(&cpu.flags)
        {
            call()(cpu, address);
        }
    })
}

fn ret() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //return from subroutine
        cpu.program_counter = CPUStack::pop_address(cpu);
    })
}

fn ret_if(condition : fn(&CPUFlags) -> bool) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        if condition(&cpu.flags)
        {
            ret()(cpu);
        }
    })
}

fn rst(address : u16) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //call subroutine at address
        call()(cpu, address);
    })
}

fn lda() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, address|
    {
        //load accumulator with value from RAM at address
        cpu.A = cpu.ram[address];
    })
}

fn ldax(from : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //load accumulator with value from register
        let address = cpu.read_register(from);
        cpu.A = cpu.ram[address];
    })
}

fn lxi(to : CPURegister) -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, value|
    {
        //load register with value
        cpu.write_register(to, value);
    })
}

fn lhld() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, address|
    {
        //load L,H registers with values from RAM at address
        cpu.L = cpu.ram[address];
        cpu.H = cpu.ram[address+1]
    })
}

fn sta() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, address|
    {
        //store accumulator into RAM at address
        cpu.ram[address] = cpu.A;
    })
}

fn stax(to : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //store accumulator into register
        cpu.write_register(to, cpu.A as u16);
    })
}

fn shld() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, address|
    {
        //store L,H registers into RAM at address
        cpu.ram[address] = cpu.L;
        cpu.ram[address+1] = cpu.H;
    })
}

fn dad(target : CPURegister) -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //add target to HL
        let old_target_value = cpu.read_register(target) as u32;
        let old_hl_value = cpu.read_register(CPURegister::HL) as u32;
        let new_hl_value = old_hl_value + old_target_value;
        cpu.write_register(CPURegister::HL, new_hl_value as u16);
        update_arithmetic_flags(cpu, new_hl_value as u16);
        cpu.flags.aux_carry = new_hl_value > 0xFFFF;
        cpu.flags.carry = cpu.flags.aux_carry;
    })
}

fn cma() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //bitwise negate accumulator
        cpu.A = !cpu.A;
    })
}

fn stc() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //set carry flag
        cpu.flags.carry = true;
    })
}

fn cmc() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //toggle carry flag
        cpu.flags.carry = !cpu.flags.carry;
    })
}

fn pchl() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //copy H,L into program counter
        let value = cpu.read_register(CPURegister::HL);
        cpu.write_register(CPURegister::ProgramCounter, value);
    })
}

fn sphl() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //copy H,L into stack pointer
        let value = cpu.read_register(CPURegister::HL);
        cpu.write_register(CPURegister::StackPointer, value);
    })
}

fn xchg() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //exchange HL with DE
        let hl = cpu.read_register(CPURegister::HL);
        let de = cpu.read_register(CPURegister::DE);
        cpu.write_register(CPURegister::HL, de);
        cpu.write_register(CPURegister::DE, hl);
    })
}

fn xthl() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //exchange HL with value at RAM address stack pointer
        let stack_pointer = cpu.stack.get_pointer();
        let stack_first_value = cpu.ram[stack_pointer];
        let stack_second_value = cpu.ram[stack_pointer+1];
        cpu.ram[stack_pointer] = cpu.L;
        cpu.ram[stack_pointer+1] = cpu.H;
        cpu.L = stack_first_value;
        cpu.H = stack_second_value;
    })
}

fn di() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //disable interrupts
        cpu.are_interrupts_enabled = false;
    })
}

fn ei() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        //enable interrupts
        cpu.are_interrupts_enabled = true;
    })
}

fn daa() -> Box<OpcodeFn>
{
    return Box::new(move |cpu|
    {
        let lsb = cpu.A & 0x0F;
        let mut msb = (cpu.A & 0xF0) >> 4;
        if lsb > 9 || cpu.flags.aux_carry
        {
            cpu.A += 6;
            cpu.flags.aux_carry = (lsb + 6) > 0x0F;
        }

        if msb > 9 || cpu.flags.carry { msb += 6; }
        cpu.A = (msb << 4) | (cpu.A & 0x0F);
        cpu.flags.aux_carry = (msb + 6) > 0x0F;
        update_arithmetic_flags(cpu, cpu.A as u16);
    })
}

fn _in() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, command|
    {
        if command == 1 { cpu.A = cpu.in1; }
        else if command == 2 { cpu.A = cpu.in2; }
        else if command == 3
        {
            let shift_amount = 8 - cpu.shift_register_offset;
            cpu.A = (cpu.shift_register >> shift_amount) as u8;
        }
    })
}

fn out() -> Box<OpcodeFnWithArg>
{
    return Box::new(move |cpu, command|
    {
        if command == 2
        {
            cpu.shift_register_offset = cpu.A & 0x07;
        }
        else if command == 4
        {
            let left = (cpu.A as u16) << 8;
            let right = cpu.shift_register >> 8;
            cpu.shift_register = left | right;
        }
    })
}
