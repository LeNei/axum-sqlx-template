import { createInertiaApp } from "@inertiajs/react";
import { createRoot } from "react-dom/client";
import "./index.css";

createInertiaApp({
  resolve: async (name) => {
    const pages = import.meta.glob("./Pages/**/*.tsx");
    return await pages[`./Pages/${name}.tsx`]();
  },
  setup({ el, App, props }) {
    createRoot(el).render(<App {...props} />);
  },
});
