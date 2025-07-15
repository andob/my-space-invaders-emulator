use core::fmt::{Display, Formatter};

pub struct CPUFlags
{
    pub negative : bool,
    pub zero : bool,
    pub reserved1 : bool,
    pub aux_carry : bool,
    pub reserved2 : bool,
    pub even : bool,
    pub reserved3 : bool,
    pub carry : bool,
}

impl CPUFlags
{
    pub fn to_byte(&self) -> u8
    {
        return ((self.negative as u8) << 7)
            | ((self.zero      as u8) << 6)
            | ((self.reserved1 as u8) << 5)
            | ((self.aux_carry as u8) << 4)
            | ((self.reserved2 as u8) << 3)
            | ((self.even      as u8) << 2)
            | ((self.reserved3 as u8) << 1)
            | ((self.carry     as u8) << 0);
    }

    pub fn from_byte(value : u8) -> CPUFlags
    {
        return CPUFlags
        {
            negative:  (value & 0b10000000) >> 7 == 1,
            zero:      (value & 0b01000000) >> 6 == 1,
            reserved1: (value & 0b00100000) >> 5 == 1,
            aux_carry: (value & 0b00010000) >> 4 == 1,
            reserved2: (value & 0b00001000) >> 3 == 1,
            even:      (value & 0b00000100) >> 2 == 1,
            reserved3: (value & 0b00000010) >> 1 == 1,
            carry:     (value & 0b00000001) >> 0 == 1,
        };
    }
}

impl Clone for CPUFlags
{
    fn clone(&self) -> Self
    {
        let byte = self.to_byte();
        return CPUFlags::from_byte(byte);
    }
}

impl Eq for CPUFlags {}
impl PartialEq<CPUFlags> for CPUFlags
{
    fn eq(&self, other : &CPUFlags) -> bool
    {
        return self.negative == other.negative &&
            self.zero == other.zero &&
            self.even == other.even &&
            self.carry == other.carry;
    }
}

impl Display for CPUFlags
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result
    {
        return write!(f, "negative:{} zero:{} even:{} carry:{}",
            self.negative, self.zero, self.even, self.carry);
    }
}
