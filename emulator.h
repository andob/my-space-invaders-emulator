#ifndef EMULATOR_H
#define EMULATOR_H

#include <memory>
#include <SDL_render.h>
#include "cpu.h"

using namespace std;

class Emulator
{
    unique_ptr<CPU> cpu;

    void render_frame(SDL_Renderer* renderer) const;

    struct EventHandlerResult { bool should_exit_frontend; };
    EventHandlerResult handle_events() const;

public:
    explicit Emulator(vector<u8>& rom_file_bytes);

    void run() const;
};

#endif //EMULATOR_H
