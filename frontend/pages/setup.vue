<script setup>
definePageMeta({
	layout: "form-page",
});

const displayname = ref("");
const username = ref("");
const password = ref("");

const api_url = useRuntimeConfig().public.api_url;

async function create_account() {
	if (
		displayname.value !== "" &&
		username.value !== "" &&
		password.value !== ""
	) {
		const body = {
			username: username.value,
			displayname: displayname.value,
			password: password.value,
		};

		const url = `${api_url}/api/setup/create_initial_user`;
		const options = {
			method: "POST",
			headers: { "content-type": "application/json" },
			body: JSON.stringify(body),
		};

		const request = await apiRequest(url, options);
		// console.log(request)

		// Error With the request (Client Side)
		if (request.is_request_error) {
			alert("An error occurred: request error");
			return;
		}

		// Error with the Response (non 200 Response code, or data says it is an error)
		if (request.is_response_error || request.data.is_error) {
			alert(`An error occurred: ${request.data.error_msg}`);
			return;
		}

		// Kontoerstellung erfolgreich
		alert(
			`Hallo ${displayname.value},\nIhr Benutzerkonto wurde erstellt: ${request.data.data}`,
		);
		return navigateTo("/");

		// biome-ignore lint/style/noUselessElse: <explanation>
	} else {
		alert("Bitte füllen Sie alle Eingabefelder aus.");
	}
}
</script>

<template>
  <h1 class="mt-5">Fahrtenbuch - Ersteinrichtung</h1>
  <p>
    Vielen Dank, dass Sie sich für
    <a href="https://github.com/AstragoDETechnologies/fahrtenbuch">Fahrtenbuch</a>
    entschieden haben. Im Folgenden wird ein Administrator-Account erstellt,
    mit dem Sie neben der Nutzung der normalen Features auch weitere
    Accounts erstellen und Verwalten können.
  </p>

  <h2>Admin-Account einrichten</h2>

  <form class="grid grid-cols-1 gap-3" @submit.prevent="create_account">
    <MyIconTextInput placeholder="Anzeigename" icon="material-symbols:id-card" v-model="displayname" />

    <MyIconTextInput placeholder="Benutzername" icon="material-symbols:person" v-model="username" />

    <MyIconTextInput :password="true" placeholder="Passwort" icon="material-symbols:key" v-model="password" />

    <MyIconButton icon="material-symbols:check-rounded">Administrator-Account erstellen
    </MyIconButton>
  </form>
</template>
