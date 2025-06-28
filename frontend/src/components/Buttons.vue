<script setup>
const emit = defineEmits(['append'])

function append(symbol) {
  emit('append', symbol)
}

function clear() {
  emit('clear')
}
</script>

<template>
  <div class="buttons-con">
    <button class="clear" @click="clear">C</button>
    <button @click="append('(')">(</button>
    <button @click="append(')')">)</button>
    <button @click="append('/')">/</button>
    <button @click="append('7')">7</button>
    <button @click="append('8')">8</button>
    <button @click="append('9')">9</button>
    <button @click="append('*')">*</button>
    <button @click="append('4')">4</button>
    <button @click="append('5')">5</button>
    <button @click="append('6')">6</button>
    <button @click="append('-')">-</button>
    <button @click="append('1')">1</button>
    <button @click="append('2')">2</button>
    <button @click="append('3')">3</button>
    <button @click="append('+')">+</button>
    <button @click="append('0')">0</button>
    <button @click="append('%')">%</button>
    <button @click="append('.')">.</button>
    <button class="equals">=</button>
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
    background: var(--ash-blue);
  }

  .clear {
    background: var(--red);
  }

  .equals {
    background: var(--green);
  }
}

@media only screen and (max-width: 600px) {
  .buttons-con {
    padding: 2.5rem 1rem 1rem 1rem;
    gap: 0.5rem;

    button {
      font-size: 2rem;
      padding: 0.6rem;
    }
  }
}
</style>
