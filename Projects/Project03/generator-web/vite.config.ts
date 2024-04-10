import { defineConfig, searchForWorkspaceRoot } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "tailwindcss";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import path from "path";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), tailwindcss(), wasm(), topLevelAwait()],
  resolve: {
    alias: {
      $lib: path.resolve("./src/lib"),
    },
  },
  optimizeDeps: {
    exclude: ["generator"],
  },
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version),
  },
  server: {
    fs: {
      // allow to serve files from outside of the workspace
      allow: [searchForWorkspaceRoot(process.cwd()), "../generator"],
    },
  },
});
