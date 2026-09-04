import { appInfo, bumpCounter, errorMessage, greet, slowTask } from "./bindings";

function el<T extends HTMLElement>(selector: string): T {
  const node = document.querySelector<T>(selector);
  if (!node) throw new Error(`missing element: ${selector}`);
  return node;
}

/** Runs a command, writing either its result or its error into `output`. */
async function show(output: HTMLElement, run: () => Promise<string>) {
  output.dataset.state = "pending";
  try {
    output.textContent = await run();
    output.dataset.state = "ok";
  } catch (error) {
    output.textContent = errorMessage(error);
    output.dataset.state = "error";
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  const greetForm = el<HTMLFormElement>("#greet-form");
  const greetInput = el<HTMLInputElement>("#greet-input");
  const greetMsg = el("#greet-msg");

  greetForm.addEventListener("submit", (event) => {
    event.preventDefault();
    show(greetMsg, () => greet(greetInput.value));
  });

  const counterOut = el("#counter-msg");
  el("#counter-inc").addEventListener("click", () =>
    show(counterOut, async () => `Counter is now ${await bumpCounter(1)}`),
  );
  el("#counter-dec").addEventListener("click", () =>
    show(counterOut, async () => `Counter is now ${await bumpCounter(-1)}`),
  );

  const asyncOut = el("#async-msg");
  el("#async-run").addEventListener("click", () => {
    asyncOut.textContent = "Working…";
    show(asyncOut, () => slowTask(1500));
  });

  // State lives in Rust, so it survives a frontend reload — unlike a JS variable.
  const info = await appInfo();
  el("#app-info").textContent =
    `${info.name} v${info.version} · Tauri ${info.tauriVersion} · ${info.os}/${info.arch}`;
});
