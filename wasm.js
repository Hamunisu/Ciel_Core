import init, { search } from "./pkg/ciel_core2.js";

async function main() {
  await init();
  const dep = document.getElementById("dep");
  const arr = document.getElementById("arr");
  const enter = document.getElementById("enter");
  const result = document.getElementById("result");

  if (!dep || !arr || !result) {
    console.error("e00001: HTMLのIDが見つかりません");
    return;
  }

  const Wasm = () => {
    const resultText = search(dep.value, arr.value);

    result.innerHTML = resultText;
  };

  if (enter) {
    enter.addEventListener("click", () => {
      Wasm();
    });
  }

  const ClickEnter = (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      enter.click();
    }
  };

  dep.addEventListener("keydown", ClickEnter);
  arr.addEventListener("keydown", ClickEnter);
}

main();
