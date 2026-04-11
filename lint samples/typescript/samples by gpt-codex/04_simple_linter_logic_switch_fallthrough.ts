function severityToColor(level: "low" | "medium" | "high"): string {
   let color = "gray";

   switch (level) {
      case "low":
         color = "green";
      case "medium":
         color = "orange";
         break;
      case "high":
         color = "red";
         break;
   }

   return color;
}
