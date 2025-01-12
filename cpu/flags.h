#ifndef FLAGS_H
#define FLAGS_H

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

#endif //FLAGS_H
