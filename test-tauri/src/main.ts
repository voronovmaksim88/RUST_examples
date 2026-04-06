let count = 0;

function render(valueEl: HTMLElement) {
  valueEl.textContent = String(count);
}

window.addEventListener("DOMContentLoaded", () => {
  const valueEl = document.querySelector("#counter-value");
  const btnMinus = document.querySelector("#btn-minus");
  const btnPlus = document.querySelector("#btn-plus");

  if (!valueEl || !btnMinus || !btnPlus) return;

  render(valueEl as HTMLElement);

  btnMinus.addEventListener("click", () => {
    count -= 1;
    render(valueEl as HTMLElement);
  });

  btnPlus.addEventListener("click", () => {
    count += 1;
    render(valueEl as HTMLElement);
  });
});
