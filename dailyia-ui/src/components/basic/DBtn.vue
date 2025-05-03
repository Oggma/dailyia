<template>
  <button class="d-btn" :disabled="disabled" @click="$emit('click')">
    <slot ></slot>
  </button>
</template>

<script setup lang="ts">
const emit = defineEmits<{
  (e: 'click'): void
}>();

const props = defineProps<{
    disabled?: boolean
}>();
</script>

<style scoped lang="scss">
@use "sass:color";

.d-btn {
    background-color: $color-bg-btn--primary;
    border: 1px solid $color-border-soft;
    border-radius: 4px;
    padding: 8px 12px;
    color: $color-text-main;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;

    &:hover {
        background-color: color.adjust($color-bg-btn--primary, $lightness: 10%);
    }

    &:disabled {
        background-color: $color-bg-disabled;
        cursor: not-allowed;
        opacity: $opacity-disabled;
    }
}
</style>
