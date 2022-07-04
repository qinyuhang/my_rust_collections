import init, {
  generate_game, get_png, next_frame, set_scale, handle_key, get_png_with_scale, resize,
} from './pkg/snake_game.js';

const scale = 10;
async function main() {

  // for requestAnimationFrame
  let tickId;

  await init();

  let [width, height] = [window.innerWidth, window.innerHeight];
  // canvas pixel width and height
  // in snake_game we render one pixel as a snake block
  let [cWidth, cHeight] = [Math.floor(width / scale), Math.floor(height / scale)]

  generate_game(cWidth, cHeight);
  // scale png up to 10
  set_scale(scale);

  let canvas = document.querySelector('#root');
  canvas.style.width = width;
  canvas.style.height = height;

  /**
   * context: canvas 2d context 
   * data: Array of u8
   * width: image width 
   * height: image height
   */
  function render(context, data, width, height) {
    let datas = Uint8ClampedArray.from(data);
    try {
      let img = new ImageData(datas, width, height);
      context.putImageData(img, 0, 0);
    } catch (e) {
      console.log(e);
      console.log(data.length, width, height);
    }
  }


  /**
   * width: canvas and image width
   * height: canvas and image height 
   * get_png: the function to get imageData
   */
  function loop(width, height, get_png) {

    canvas.width = width;
    canvas.height = height;

    let context = canvas.getContext('2d');


    function tick() {
      let data = get_png();

      next_frame();
      render(context, data, width, height);
      return requestAnimationFrame(tick);
    }

    tickId = tick();

  }

  // loop(cWidth, cHeight, get_png);
  loop(cWidth * scale, cHeight * scale, get_png_with_scale);

  let timerId;

  function handleResize() {
    window.cancelAnimationFrame(tickId);
    tickId = null;
    let [width, height] = [window.innerWidth, window.innerHeight];
    let [cw, ch] = [Math.floor(width / scale), Math.floor(height / scale)];
    canvas.style.width = width;
    canvas.style.height = height;
    canvas.width = cw * scale;
    canvas.height = ch * scale;

    resize(cw * scale, ch * scale);

    if (timerId) {
      clearTimeout(timerId);
    }
    // loop is called to many times it must has a throttle
    timerId = setTimeout(() => {
      loop(cw * scale, ch * scale, get_png_with_scale);
    }, 2000);

  }

  window.addEventListener('resize', handleResize);

  window.addEventListener('keydown', (e) => {
    console.log(e.key)

    // 0 - KeyUp
    // 1 - KeyDown
    // 2 - KeyLeft
    // 3 - KeyRight
    // 4 - Space
    // 5 - Enter
    let r = 0;

    switch (e.key) {
      case 'ArrowUp':
      case 'w':
        r = 0;
        break;
      case 'ArrowDown':
      case 's':
        r = 1;
        break;
      case 'ArrowLeft':
      case 'a':
        r = 2;
        break;
      case 'ArrowRight':
      case 'd':
        r = 3;
        break;
      case ' ':
        console.log('is empty');
        r = 4;
        break;
      case 'Enter':
        r = 5;
        break;
      // other key not response
      default:
        return;
    }

    handle_key(r);

  })

}
window.onload = main;
