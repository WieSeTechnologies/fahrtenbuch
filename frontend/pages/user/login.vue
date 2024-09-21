<script setup>
definePageMeta({
    layout: "form-page"
});

const username = ref("");
const password = ref("");

const api_url = useRuntimeConfig().public.api_url;

async function login() {
    if (username.value !== "" && password.value !== "") {
        const body = { username: username.value, password: password.value };

        const url = `${api_url}/api/user/login`;
        const options = {
            method: 'POST',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify(body)
        };

        const request = await apiRequest(url, options);
        if (!request.is_error && !request.data.is_error && request.data.data != null) {
            alert(`Sie wurden mit der Session-ID: ${request.data.data.session_id} angemeldet.`)
        }

    } else {
        alert("Bitte füllen Sie alle Eingabefelder aus.")
    }
}
</script>


<template>
    <h1 class="mt-5">Anmeldung</h1>

    <p>Wenn Sie bereits ein Benutzerkonto besitzen, können Sie sich hier anmelden.</p>

    <form class="grid grid-cols-1 gap-3" @submit.prevent="login">
        <MyIconTextInput placeholder="Benutzername" icon="material-symbols:person" v-model="username" />

        <MyIconTextInput :password="true" placeholder="Passwort" icon="material-symbols:key" v-model="password" />

        <MyIconButton icon="material-symbols:login">Anmelden</MyIconButton>
    </form>
</template>