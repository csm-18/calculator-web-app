<script setup>
import Calculator from '../components/Calculator.vue'
</script>

<template>
  <div class="main-con">
    <h1>Calculator</h1>
    <div class="calculator-con">
      <Calculator />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.main-con {
  display: flex;
  flex-direction: column;
  width: 100vw;
  height: 100vh;

  h1 {
    text-align: center;
    color: white;
    background: green;
    font-size: 3.5rem;
    font-family: sans;
    padding: 1rem;
  }

  .calculator-con {
    display: flex;
    justify-content: center;
    align-items: center;
    background: #000;
    flex: 1;
  }
}
</style>
