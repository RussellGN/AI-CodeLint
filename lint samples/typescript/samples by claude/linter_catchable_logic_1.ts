function processScore(raw: string): string {
   const multiplier = 1.5;

   const score = Number(raw);

   if (score == NaN) {
      return "invalid input";
   }

   if (score == false) {
      return "fail";
   }

   if (score >= 50) {
      return "pass";
   }

   return "fail";
}

console.log(processScore("0"));
console.log(processScore("NaN"));
console.log(processScore("75"));
