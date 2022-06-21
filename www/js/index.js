import { Ai } from "adversarial-2048";

const ai = Ai.new(11);
console.log(ai);
console.log(typeof(ai));

window.requestAnimationFrame(function () {
  new GameManager(4, KeyboardInputManager, HTMLActuator, LocalStorageManager, ai);
});
