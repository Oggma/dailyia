import { fileURLToPath, URL } from "node:url";

import vueDevTools from "vite-plugin-vue-devtools";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd());

  return {
    plugins: [vue(), vueDevTools(), tailwindcss()],
    server: {
      port: parseInt(env.VITE_APP_PORT) || 5173,
    },
  };
});
