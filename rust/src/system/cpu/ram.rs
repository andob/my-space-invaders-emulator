use std::ops::{Index, IndexMut};

pub const RAM_SIZE : usize = 0x10000; //64KB

pub struct RAM
{
    data : Box<[u8; RAM_SIZE]>
}

impl RAM
{
    pub fn new() -> RAM
    {
        return RAM { data: Box::new([0; RAM_SIZE]) };
    }
}

impl Index<u16> for RAM
{
    type Output = u8;

    fn index(&self, address : u16) -> &u8
    {
        return &self.data[(address as usize) % RAM_SIZE];
    }
}

impl IndexMut<u16> for RAM
{
    fn index_mut(&mut self, address : u16) -> &mut u8
    {
        return &mut self.data[(address as usize) % RAM_SIZE];
    }
}
