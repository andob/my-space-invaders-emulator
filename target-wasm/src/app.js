import init, * as emulator from './target_wasm.js';

(async () =>
{
    await init();
    emulator.initialize();

    show_about_dialog();

    window.onkeydown = event => emulator.on_key_down(event.key);
    window.onkeyup = event => emulator.on_key_up(event.key);

    emulate();
})()

function emulate()
{
    emulator.render_next_frame();
    requestAnimationFrame(emulate);
}

function show_about_dialog()
{
    alert('Space Invaders Arcade Machine Emulator\n\n' +
        'Controls:\n' +
        'C = Insert coin\n' +
        '1 = Select one player\n' +
        '2 = Select two players\n' +
        'Left / Right = Player 1 move\n' +
        'Space = Player 1 shoot\n' +
        'A / D = Player 2 move\n' +
        'S = Player 2 shoot');
}
