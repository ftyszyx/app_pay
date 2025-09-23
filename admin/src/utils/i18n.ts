import { createI18n } from "vue-i18n";
import enUS from "@/locales/en-US.json";
import zhCN from "@/locales/zh-cn.json";

export const i18n = createI18n({
  legacy: false,
  locale: (localStorage.getItem("locale") as "en" | "zh-cn") || "zh-cn",
  fallbackLocale: "zh-cn",
  messages: { en: enUS, "zh-cn": zhCN },
});
