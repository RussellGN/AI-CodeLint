import * as vscode from "vscode";
import * as lsp from "vscode-languageclient/node";
import { checkAndReportIfOutdated } from "./lib";

let client: lsp.LanguageClient;

export async function activate(_context: vscode.ExtensionContext) {
   if ((await checkAndReportIfOutdated()) == "outdated") {
      return;
   }

   if (!process.env.OPENROUTER_API_KEY) {
      vscode.window.showErrorMessage("OPENROUTER_API_KEY environment variable is required to use this extension. Please set it globally on your machine. Aborting...");
      return;
   }

   const traceOutputChannel = vscode.window.createOutputChannel("AI CodeLint trace");
   const command = `${process.env.SERVER_PATH || "ai-codelint"}`;

   const run: lsp.Executable = {
      command,
      args: ["--mode", "server"],
      options: {
         env: {
            ...process.env,
            // eslint-disable-next-line @typescript-eslint/naming-convention
            RUST_LOG: "debug",
         },
      },
   };
   const serverOptions: lsp.ServerOptions = {
      run,
      debug: run,
   };

   const clientOptions: lsp.LanguageClientOptions = {
      documentSelector: [{ scheme: "file", language: "typescript" }],
      traceOutputChannel,
      connectionOptions: {
         maxRestartCount: 0,
      },
   };
   client = new lsp.LanguageClient("ai-codelint", "AI CodeLint", serverOptions, clientOptions);
   client.start();
}

export function deactivate(): Thenable<void> | undefined {
   if (!client) {
      return undefined;
   }
   return client.stop();
}
