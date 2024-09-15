<script setup>
const props = defineProps({
  icon: {
    type: String,
    required: true,
  },
  placeholder: {
    type: String,
    required: true,
  },
  password: {
    required: false,
  },
});
const { icon, placeholder } = props;
</script>

<template>
  <label class="input input-bordered flex items-center gap-2">
    <Icon :name="icon" class="h-4 w-4 opacity-70" />
    <input type="text" class="grow" :placeholder="placeholder" />
  </label>
</template>
