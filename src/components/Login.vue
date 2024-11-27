<template>
  <div>
    <h2>Login</h2>
    <input v-model="email" placeholder="Email" />
    <input v-model="password" type="password" placeholder="Password" />
    <button @click="login">Login</button>
    <p>{{ message }}</p>
  </div>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";

export default {
  name: "Login",
  data() {
    return {
      email: "",
      password: "",
      message: "",
    };
  },
  methods: {
    async login() {
      try {
        const response = await invoke("authenticate", {
          email: this.email,
          password: this.password,
        });
        this.message = response;
        this.$emit("authenticated", true);
      } catch (error) {
        this.message = `Error: ${error}`;
      }
    },
  },
};
</script>

<style scoped>

</style>
