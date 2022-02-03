import { UniverseImpl } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = UniverseImpl.new(JSON.stringify({cells:[[0,1,0],[0,0,1],[1,1,1],[0,0,0]]}));


const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();
    if (!universe.complete){
        requestAnimationFrame(renderLoop);
    }
     
  };

  requestAnimationFrame(renderLoop);