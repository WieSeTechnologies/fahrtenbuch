<script setup>
const props = defineProps({
	icon: {
		type: String,
		required: true,
	},
});
const { icon } = props;
</script>

<template>
  <button class="btn">
    <Icon :name="icon" class="h-6 w-6" />
    <slot />
  </button>
</template>
