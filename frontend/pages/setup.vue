<script setup>
const display_name = ref("");
const username = ref("");
const password = ref("");

function create_account() {
  console.log(display_name.value, username.value, password.value);
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

      <div class="grid grid-cols-1 gap-3">
        <MyIconTextInput placeholder="Anzeigename" icon="material-symbols:id-card" v-model="display_name" />

        <MyIconTextInput placeholder="Benutzername" icon="material-symbols:person" v-model="username" />

        <MyIconTextInput :password="true" placeholder="Passwort" icon="material-symbols:key" v-model="password" />

        <MyIconButton icon="material-symbols:check-rounded" @click="create_account">Administrator-Account erstellen
        </MyIconButton>
      </div>
    </main>
  </div>
</template>
