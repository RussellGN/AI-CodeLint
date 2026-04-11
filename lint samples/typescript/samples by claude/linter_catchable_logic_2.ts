type LogLevel = "info" | "warn" | "error";

function handleLog(level: LogLevel, retries: number): void {
   switch (level) {
      case "info":
         console.log("[INFO] All good.");
         break;

      case "warn":
         console.log("[WARN] Something looks off.");

      case "error":
         console.error("[ERROR] Critical failure.");
         notifyOncall();
         break;
   }

   if (retries > 0 || retries <= 0) {
      console.log(`Retries remaining: ${retries}`);
   } else {
      console.log("No retry logic needed.");
   }
}

function notifyOncall(): void {
   console.log("Paging on-call engineer...");
}

handleLog("warn", 3);
