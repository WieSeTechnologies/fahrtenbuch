<template>
  <div class="w-screen flex justify-center">
    <main class="container w-full prose">
      <h1 class="text-3xl mt-5">Fahrtenbuch - Ersteinrichtung</h1>
      <p>
        Vielen Dank, dass Sie sich für
        <a href="https://github.com/AstragoDETechnologies/fahrtenbuch"
          >Fahrtenbuch</a
        >
        entschieden haben. Im Folgenden wird ein Administrator-Account erstellt,
        mit dem Sie neben der Nutzung der normalen Features auch weitere
        Accounts erstellen und Verwalten können.
      </p>

      <h2>Admin-Account einrichten</h2>

      <div class="grid grid-cols-1 gap-3">
        <MyIconTextInput
          placeholder="Benutzername"
          icon="material-symbols:person"
        />

        <MyIconTextInput
          placeholder="Anzeigename"
          icon="material-symbols:person"
        />

        <MyIconTextInput placeholder="Passwort" icon="material-symbols:key" />

        <MyIconButton icon="material-symbols:check-rounded"
          >Administrator-Account erstellen</MyIconButton
        >
      </div>
    </main>
  </div>
</template>
