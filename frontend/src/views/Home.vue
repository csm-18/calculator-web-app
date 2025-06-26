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
    background: white;
    color: var(--blue);
    font-size: 4.5rem;
    padding: 0.5rem;
    font-family: 'Gruppo', sans-serif;
    font-weight: 600;
    font-style: normal;
  }

  .calculator-con {
    display: flex;
    justify-content: center;
    align-items: center;
    background: white;
    flex: 1;
  }
}
</style>
