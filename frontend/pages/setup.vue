<script setup>
const displayname = ref("");
const username = ref("");
const password = ref("");

const api_url = useRuntimeConfig().public.api_url;

async function create_account() {
  console.log(displayname.value, username.value, password.value);
  if (displayname.value !== "" && username.value !== "" && password.value !== "") {
    const body = { username: username.value, displayname: displayname.value, password: password.value };

    const url = `${api_url}/api/setup/create_initial_user`;
    const options = {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(body)
    };

    try {
      const response = await fetch(url, options);
      const data = await response.text();
      console.log(data);
      if (!response.ok || !response) {
        alert(`Fehler beim Erstellen des Accounts: ${data}`);
      }
    } catch (error) {
      console.error(error);
      alert(`Fehler beim Erstellen des Accounts: ${error}`);
    }
  } else {
    alert("Bitte füllen Sie alle Eingabefelder aus.")
  }
}

</script>

<template>
  <div class="w-screen flex justify-center">
    <main class="container w-full prose">
      <h1 class="text-3xl mt-5">Fahrtenbuch - Ersteinrichtung</h1>
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
    </main>
  </div>
</template>
