import * as vscode from "vscode";
import * as lsp from "vscode-languageclient/node";

let client: lsp.LanguageClient;

export async function activate(_context: vscode.ExtensionContext) {
   const traceOutputChannel = vscode.window.createOutputChannel("AI CodeLint trace");
   const command = process.env.SERVER_PATH || "ai-codelint";

   const run: lsp.Executable = {
      command,
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
      documentSelector: [{ scheme: "file", language: "ts" }],
      traceOutputChannel,
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
