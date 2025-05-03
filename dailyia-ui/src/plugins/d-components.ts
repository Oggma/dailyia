import type { App } from "vue";
import DTextField from "@/components/basic/DTextField.vue";
import DBtn from "@/components/basic/DBtn.vue";

export function registerComponents(app: App) {
  app.component("DTextField", DTextField);
  app.component("DBtn", DBtn);
}
