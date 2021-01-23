import * as wasm from "compound_interest_calculator";

const submitButton = document.getElementById("submit");
const principleInput = document.getElementById("principle");
const rateInput = document.getElementById("rate");
const yearsInput = document.getElementById("years");
const resultContainer = document.getElementById("result");

submitButton.addEventListener("click", event => {
  const principle = principleInput.value;
  const rate = rateInput.value;
  const years = yearsInput.value;

  console.log({principle, rate, years});

  const result = wasm.compound_interest_formatted(principle, rate, years, 1);

  resultContainer.innerHTML = result;
});
