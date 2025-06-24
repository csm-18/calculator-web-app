<script setup>
import Buttons from '@/components/Buttons.vue'
</script>

<template>
  <div class="calc-con">
    <div class="display">
      <p class="result">0.0</p>
      <input class="input" type="text" />
    </div>
    <Buttons />
  </div>
</template>

<style lang="scss" scoped>
.calc-con {
  display: flex;
  flex-direction: column;
  background: greenyellow;
  padding: 1rem;
}
</style>
