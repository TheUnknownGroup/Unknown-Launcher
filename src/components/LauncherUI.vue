<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import {ref} from "vue";

const message = ref("");

export default {
  name: "LauncherUI",
  data() {
    return {
      username: "",
      password: "",
      version: "",
      message: "",
    };
  },
  methods: {
    async launchGame() {
      try {
        const response = await invoke("launch_game", {
          username: this.username,
          version: this.version,
        });
        this.message = response;
      } catch (error) {
        this.message = `Error: ${error}`;
      }
    },
  },
};
</script>

<template>
  <div>
    <input v-model="username" placeholder="Username" />
    <input v-model="password" placeholder="Password" />
    <button onclick="launchGame()">Launch Game</button>
    <p>{{ message }}</p>
  </div>
</template>

<style scoped>

</style>