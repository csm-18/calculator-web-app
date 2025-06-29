<script setup>
import { onMounted, ref } from 'vue'

import Buttons from '@/components/Buttons.vue'

import init, { calc } from '../pkg/calc.js'

const expression = ref('')
const result = ref('0')

function backspace() {
  expression.value = expression.value.slice(0, -1)
}

function append(symbol) {
  expression.value = expression.value + symbol
}

function clear() {
  expression.value = ''
  result.value = '0'
}

//wasm
const wasmLoaded = ref(false)
onMounted(async () => {
  try {
    await init()
    wasmLoaded.value = true
  } catch (error) {
    console.error('Error while loading wasm module!')
  }
})
function equals() {
  if (wasmLoaded) {
    result.value = calc(expression.value)
  }
}
</script>

<template>
  <div class="calc-con">
    <div class="input-con">
      <input class="input" v-model="expression" type="text" autofocus inputmode="none" />
    </div>
    <p class="result">{{ result }}</p>
    <div class="backspace-con">
      <button class="backspace" @click="backspace">BS</button>
    </div>
    <Buttons @append="append" @clear="clear" @equals="equals" />
  </div>
</template>

<style lang="scss" scoped>
.calc-con {
  background: var(--blue);
  padding: 1rem;
  border-radius: 12px;

  .result {
    text-align: end;
    font-size: 2rem;
    padding: 0 1rem;
    color: black;
  }

  .input-con {
    width: 100%;
    padding-bottom: 1rem;

    .input {
      background: transparent;
      max-width: 18rem;
      text-align: end;
      padding: 0.5rem 1rem;
      border: 0;
      font-size: 2.5rem;
      caret-color: var(--light-gray-blue);
      color: black;
    }
    .input:focus {
      outline: none;
    }
  }
  .backspace-con {
    display: flex;
    justify-content: end;
    padding-top: 1.5rem;
    padding-right: 0.5rem;

    .backspace {
      font-size: 1.8rem;
      padding: 0.1rem 0.5rem;
      border-radius: 50px;
      background: var(--light-orange);
      border: 0;
    }
  }
}

@media only screen and (max-width: 600px) {
  .calc-con {
    border-radius: 0;

    .input-con {
      .input {
        font-size: 3rem;
      }
    }

    .backspace-con {
      padding-top: 2rem;
      .backspace {
        padding: 0.1rem 0.8rem;
      }
    }
  }
}
</style>
