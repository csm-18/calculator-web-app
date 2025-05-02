function Clear() {
  document.getElementById("input").value = "";
}

function Backspace() {
  let input = document.getElementById("input");
  input.value = input.value.slice(0, -1);
}

function NumberOrSymbolClick(value) {
  document.getElementById("input").value += value;
}
