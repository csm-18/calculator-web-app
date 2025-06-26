<script setup></script>

<template>
  <div class="buttons-con">
    <button>C</button>
    <button>(</button>
    <button>)</button>
    <button>/</button>
    <button>7</button>
    <button>8</button>
    <button>9</button>
    <button>*</button>
    <button>4</button>
    <button>5</button>
    <button>6</button>
    <button>-</button>
    <button>1</button>
    <button>2</button>
    <button>3</button>
    <button>+</button>
    <button>0</button>
    <button>%</button>
    <button>.</button>
    <button>=</button>
  </div>
</template>

<style lang="scss" scoped>
.buttons-con {
  padding-top: 1rem;
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1rem;

  button {
    font-size: 2rem;
    padding: 0.8rem;
    border: 0;
    border-radius: 50%;
    background: white;
  }
}
</style>
