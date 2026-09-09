import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
  site: "https://ther12k.github.io",
  base: "/velqu",
  integrations: [
    starlight({
      title: "Velqu",
      description:
        "A Rust HTTP runtime running TypeScript handlers on quickjs-ng — one schema contract driving types, validation, Treaty clients, OpenAPI, and the contract lock.",
      logo: { src: "./public/favicon.svg" },
      social: [
        { icon: "github", label: "GitHub", href: "https://github.com/ther12k/velqu" },
        { icon: "npm", label: "npm", href: "https://www.npmjs.com/org/velqu" },
      ],
      components: {
        // Header-right navigation: plain text links + the default social icons.
        SocialIcons: "./src/components/SocialIcons.astro",
      },
      sidebar: [
        { label: "Start", items: ["getting-started", "deployment"] },
        { label: "Targets", items: ["browser-wasm"] },
        { label: "Project", items: ["known-limitations"] },
      ],
    }),
  ],
});
