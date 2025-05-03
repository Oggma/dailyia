import "@/assets/styles/global.scss";
import "@/assets/styles/fonts.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import { registerComponents } from "./plugins/d-components";

import App from "./App.vue";
import router from "./router";

const app = createApp(App);

app.use(createPinia());
app.use(router);
registerComponents(app);

app.mount("#app");
