import { defineConfig } from 'astro/config';
import tailwind from "@astrojs/tailwind";
import react from "@astrojs/react";
// import htmx from './htmx-plugin';

// https://astro.build/config
export default defineConfig({
  integrations: [tailwind(), react()]
});