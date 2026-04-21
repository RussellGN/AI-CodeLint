import { invoke } from "@tauri-apps/api/core";

export default function useLinter() {
   async function lintContents(contents: string) {
      try {
         const res = await invoke<string>("lint_contents", { contents });
         return res;
      } catch (error) {
         return "An error occurred while linting. " + (error instanceof Error ? error.message : String(error));
      }
   }

   return { lintContents };
}
