<!-- TODO: Textfeld reagiert noch nicht auf Änderungen von außerhalb. Reaktivität muss wiederhergestellt werden. Gibt es eine bessere Methode als Events? -->
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
		type: Boolean,
		required: false,
		default: false,
	},
});
const { icon, placeholder, password } = props;
</script>

<template>
  <label class="input input-bordered flex items-center gap-2">
    <Icon :name="icon" class="h-4 w-4 opacity-70" />
    <input :type="password ? 'password' : 'text'" class="grow" :placeholder="placeholder"
      @input="$emit('update:modelValue', $event.target.value)" />
  </label>
</template>
