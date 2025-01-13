#include "emulator.h"
#include "constants.h"
#include <SDL.h>
#include <iostream>

Emulator::Emulator(vector<u8>& rom_file_bytes)
{
    this->cpu = make_unique<CPU>();

    ranges::copy(rom_file_bytes, this->cpu->ram->begin());
}

#define HandleSDLError(condition, function) if ((condition) != 0) \
{ \
    cout << "Error in " << #function << ": " << SDL_GetError() << endl; \
    return; \
}

void Emulator::run() const
{
    constexpr u32 window_width = BLOCK_WIDTH * DISPLAY_WIDTH;
    constexpr u32 window_height = BLOCK_HEIGHT * DISPLAY_HEIGHT;

    i32 error = SDL_Init(SDL_INIT_VIDEO);
    HandleSDLError(error, SDL_Init);

    SDL_Window* window = SDL_CreateWindow(/*title*/ "Space Invaders",
        /*position*/ SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
        /*size*/ window_width, window_height,
        /*flags*/ SDL_WINDOW_SHOWN | SDL_WINDOW_OPENGL);
    HandleSDLError(!window, SDL_CreateWindow);

    SDL_Renderer* renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
    HandleSDLError(!renderer, SDL_CreateRenderer);

    error = SDL_InitSubSystem(SDL_INIT_EVENTS);
    HandleSDLError(error, SDL_InitSubSystem);

    while (true)
    {
        //todo run cpu cycles
        for (int i = 0; i <= 1000; i++)
            this->cpu->tick();

        this->render_frame(renderer);

        //todo interrupt cpu
        // this->cpu->interrupt(1);
        // this->cpu->interrupt(2);

        const EventHandlerResult result = this->handle_events();
        if (result.should_exit_frontend)
        {
            SDL_DestroyRenderer(renderer);
            SDL_DestroyWindow(window);
            SDL_Quit();
            return;
        }
    }
}

void Emulator::render_frame(SDL_Renderer* renderer) const
{
    i32 error = SDL_SetRenderDrawColor(renderer, 0x00, 0x00, 0x00, 0xFF);
    HandleSDLError(error, SDL_SetRenderDrawColor);

    error = SDL_RenderClear(renderer);
    HandleSDLError(error, SDL_RenderClear);

    error = SDL_SetRenderDrawColor(renderer, 0xFF, 0xFF, 0xFF, 0xFF);
    HandleSDLError(error, SDL_SetRenderDrawColor);

    //todo how is display rendered?
    u16 display_address = DISPLAY_START_ADDRESS;
    for (u16 ix = 0; ix < DISPLAY_WIDTH / 8; ix++)
    {
        for (u16 iy = 0; iy < DISPLAY_HEIGHT; iy++)
        {
            u8 byte = (*this->cpu->ram)[(display_address++) % RAM_SIZE];
            // cout << uppercase << hex << static_cast<int>(display_address) << ' ' << static_cast<int>(byte) << endl;
            if (!byte) continue;
            else
            {
                cout << "HERE!";
            }
            for (u8 bit_index = 0; bit_index < 8; bit_index++)
            {
                const u8 bit = byte & 1;
                byte >>= 1;
                if (!bit) continue;
                cout << "HERE!";

                SDL_Rect rectangle;
                rectangle.x = ix * BLOCK_WIDTH;
                rectangle.y = (iy + bit_index) * BLOCK_HEIGHT;
                rectangle.w = BLOCK_WIDTH;
                rectangle.h = BLOCK_HEIGHT;
                error = SDL_RenderFillRect(renderer, &rectangle);
                HandleSDLError(error, SDL_RenderFillRect);
            }
        }
    }

    SDL_RenderPresent(renderer);
    SDL_Delay(17);
}

Emulator::EventHandlerResult Emulator::handle_events() const
{
    SDL_Event event;
    while (SDL_PollEvent(&event))
    {
        if ((event.type == SDL_QUIT) || (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE))
        {
            return EventHandlerResult { .should_exit_frontend = true };
        }
    }

    return EventHandlerResult { .should_exit_frontend = false };
}
