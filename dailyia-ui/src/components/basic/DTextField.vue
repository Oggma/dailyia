<template>
  <div class="d-input--wrapper">
    <input class="d-input--input" type="text" :value="modelValue" @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)" />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  modelValue: string;
}>();


</script>
<style scoped lang="scss">
.d-input {
  &--wrapper {
   
  }
  &--input {
    border: 1px solid $color-border-soft;
    border-radius: 4px;
    padding: 8px 12px;
    background-color: $color-bg-input;

  }
}
</style>
