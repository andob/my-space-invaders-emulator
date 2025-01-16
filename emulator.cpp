#include "emulator.h"

#include <bitset>

#include "constants.h"
#include <SDL.h>
#include <iostream>
#include <vector>

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
        for (int i = 0; i <= 16600; i += this->cpu->tick()) {}
        this->cpu->interrupt(1);

        for (int i = 0; i <= 16600; i += this->cpu->tick()) {}
        this->cpu->interrupt(2);

        this->render_frame(renderer);

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

    u16 display_address = DISPLAY_START_ADDRESS;
    for (u16 ix = 0; ix < DISPLAY_WIDTH; ix++)
    {
        for (u16 iy = 0; iy < DISPLAY_HEIGHT; iy += 8)
        {
            u8 byte = (*this->cpu->ram)[(display_address++) % RAM_SIZE];
            for (u8 bit_index = 0; bit_index < 8; bit_index++)
            {
                const u8 bit = byte & 1;
                byte >>= 1;
                if (!bit) continue;

                u8 r = 0x00, g = 0x00, b = 0x00;
                if (iy > 200 && iy < 220) r = 0xFF;
                else if (iy < 80) g = 0xFF;
                else r = g = b = 0xFF;

                error = SDL_SetRenderDrawColor(renderer, r, g, b, 0xFF);
                HandleSDLError(error, SDL_SetRenderDrawColor);

                SDL_Rect rectangle;
                rectangle.x = ix * BLOCK_WIDTH;
                rectangle.y = (DISPLAY_HEIGHT - iy - bit_index) * BLOCK_HEIGHT;
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
        if (event.type == SDL_QUIT || (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE))
        {
            return EventHandlerResult { .should_exit_frontend = true };
        }

        if (event.type == SDL_KEYDOWN || event.type == SDL_KEYUP)
        {
            u8 player = 1, key = 0;
            switch (event.key.keysym.sym)
            {
                case SDLK_c: key = 0; break; //insert coin
                case SDLK_1: case SDLK_KP_1: key = 2; break; //1 player
                case SDLK_2: case SDLK_KP_2: key = 1; break; //2 players
                case SDLK_SPACE: player = 1; key = 4; break; //player 1 shoot
                case SDLK_LEFT: player = 1; key = 5; break; //player 1 left
                case SDLK_RIGHT: player = 1; key = 6; break; //player 1 right
                case SDLK_s: player = 2; key = 4; break; //player 2 shoot
                case SDLK_a: player = 2; key = 5; break; //player 2 left
                case SDLK_d: player = 2; key = 6; break; //player 2 right
                default: continue;
            }

            if (player == 1 && event.type == SDL_KEYDOWN) this->cpu->in1 |= 1 << key;
            else if (player == 1 && event.type == SDL_KEYUP) this->cpu->in1 &= ~(1 << key);
            else if (player == 2 && event.type == SDL_KEYDOWN) this->cpu->in2 |= 1 << key;
            else if (player == 2 && event.type == SDL_KEYUP) this->cpu->in2 &= ~(1 << key);
        }
    }

    return EventHandlerResult { .should_exit_frontend = false };
}

#undef HandleSDLError
