#include <fstream>
#include <iostream>
#include <memory>
#include <vector>
#include "constants.h"
#include "cpu.h"
#include "emulator.h"

using namespace std;

int main(const int argc, char** argv)
{
    if (argc <= 1)
    {
        cout << "Syntax: spaceinvaders <romfile>" << endl;
        return EXIT_FAILURE;
    }

    const char* rom_file_path = argv[argc-1];
    ifstream rom_file_stream(rom_file_path, ios_base::in | ios_base::binary);
    if (!rom_file_stream.good())
    {
        cout << "ROM file does not exist!" << endl;
        return EXIT_FAILURE;
    }

    rom_file_stream.seekg(0, ios_base::end);
    const size_t rom_file_length = rom_file_stream.tellg();
    rom_file_stream.seekg(0, ios_base::beg);

    vector<u8> rom_file_bytes;
    rom_file_bytes.reserve(rom_file_length);
    copy( istreambuf_iterator(rom_file_stream),
        istreambuf_iterator<char>(),
        back_inserter(rom_file_bytes));

    unique_ptr<Emulator> emulator = make_unique<Emulator>(rom_file_bytes);
    emulator->run();

    return EXIT_SUCCESS;
}
