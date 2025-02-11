use crate::system::cpu::CPU;
use crate::system::cpu::stack::CPUStack;

pub struct CPUInterrupts {}
impl CPUInterrupts
{
    pub fn interrupt(cpu : &mut CPU, number : u16)
    {
        if cpu.are_interrupts_enabled
        {
            cpu.are_interrupts_enabled = false;
            CPUStack::push_address(cpu, cpu.program_counter);

            let interrupt_vector = number << 3;
            cpu.program_counter = interrupt_vector;
        }
    }
}
