function makeCounters(): (() => number)[] {
   const counters = [];
   for (var i = 0; i < 5; i++) {
      counters.push(() => i);
   }
   return counters;
}
