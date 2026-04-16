import axios from "axios";
import * as settings from "../package.json";
import * as vscode from "vscode";

export async function checkAndReportIfOutdated(): Promise<"outdated" | "not outdated"> {
   const res = await axios<{ recommended_version: string }>("https://raw.githubusercontent.com/RussellGN/AI-CodeLint/refs/heads/main/status.json");

   if (res.data.recommended_version.toString() !== settings.version.toString()) {
      vscode.window.showErrorMessage(
         `Current version '${settings.version}' of ${BINARY_NAME} is out of date. Please download and use the recommended version '${res.data.recommended_version}' or newer.`,
      );
      return "outdated";
   }

   return "not outdated";
}

export const BINARY_NAME = "ai-codelint";
