#ifndef CONSTANTS_H
#define CONSTANTS_H

#define RAM_SIZE 0x10000 //64KB

#define DISPLAY_WIDTH 256
#define DISPLAY_HEIGHT 224
#define DISPLAY_START_ADDRESS 0x2400
#define DISPLAY_END_ADDRESS 0x4000
#define BLOCK_WIDTH 3
#define BLOCK_HEIGHT 3

typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned int u32;
typedef unsigned long u64;

typedef char i8;
typedef short i16;
typedef int i32;
typedef long i64;

#endif //CONSTANTS_H
