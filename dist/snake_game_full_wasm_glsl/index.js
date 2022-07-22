import init, {
  // get_is_finshed
} from './pkg/snake_game_full_wasm_glsl.js';

// window.get_is_finshed = get_is_finshed;

async function main() {
  await init();
}
window.onload = main;
