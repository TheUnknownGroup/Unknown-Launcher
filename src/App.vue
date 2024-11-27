<template>
  <div id="app">
    <h1>Unknown Launcher</h1>
    <Login v-if="!authenticated" @authenticated="authenticated = true" />
    <LauncherUI v-else/>
  </div>
</template>

<script lang="ts">
import Login from "./components/Login.vue"
import LauncherUI from "./components/LauncherUI.vue"

export default {
  name: "App",
  components: {
    Login,
    LauncherUI,
  },
  data() {
    return {
      authenticated: false,
    }
  }
};
</script>

<style>

</style>