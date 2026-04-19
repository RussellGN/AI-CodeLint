import { constants as fsConstants } from "node:fs";
import { access } from "node:fs/promises";
import { delimiter, join } from "node:path";
import * as vscode from "vscode";

export const BINARY_NAME = "ai-codelint";
export const WEBSITE = "https://russellgn.github.io/AI-CodeLint";

async function isExecutable(path: string): Promise<boolean> {
   try {
      await access(path, fsConstants.X_OK);
      return true;
   } catch {
      return false;
   }
}

async function resolveBinary(command: string): Promise<string | undefined> {
   if (command.includes("/") || command.includes("\\")) {
      return (await isExecutable(command)) ? command : undefined;
   }

   const pathEntries = (process.env.PATH || "").split(delimiter).filter(Boolean);
   const candidates = [command];

   if (process.platform === "win32" && !command.includes(".")) {
      const pathExts = (process.env.PATHEXT || ".EXE;.CMD;.BAT;.COM").split(";").filter(Boolean);
      for (const ext of pathExts) {
         candidates.push(`${command}${ext}`);
      }
   }

   for (const pathEntry of pathEntries) {
      for (const candidate of candidates) {
         const candidatePath = join(pathEntry, candidate);
         if (await isExecutable(candidatePath)) {
            return candidatePath;
         }
      }
   }

   return undefined;
}

export async function checkAndReportIfBinaryMissing(command: string): Promise<"missing" | "found"> {
   if (await resolveBinary(command)) {
      return "found";
   }

   let setupHint =
      command === BINARY_NAME
         ? `Install '${BINARY_NAME}' and make sure it is on your PATH, or set SERVER_PATH to the full binary path.`
         : `The SERVER_PATH value '${command}' could not be resolved. Set SERVER_PATH to a valid executable path.`;

   vscode.window.showErrorMessage(`Could not start AI CodeLint because the '${command}' executable was not found. ${setupHint}`, "View Installation Guide").then((choice) => {
      if (choice === "View Installation Guide") {
         vscode.env.openExternal(vscode.Uri.parse(WEBSITE));
      }
   });

   return "missing";
}
