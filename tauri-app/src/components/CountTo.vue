<script setup lang="ts">
import gsap from "gsap"
import {reactive, watch} from "vue";
const active = reactive({
  num: 0
})

const props = defineProps({
  value: {
    type: Number,
    default: 0
  }
})

const AnimateToValue = () => {
  gsap.to(active, {
    duration: 0.5,
    num: props.value
  })
}

watch(() => props.value, () => AnimateToValue())
</script>

<template>
  {{ props.value.toFixed(0) }}
</template>
