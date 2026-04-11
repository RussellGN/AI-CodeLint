function binarySearch(arr: number[], target: number): number {
   let lo = 0;
   let hi = arr.length;

   while (lo < hi) {
      const mid = Math.floor((lo + hi) / 2);

      if (arr[mid] === target) {
         return mid;
      } else if (arr[mid] < target) {
         lo = mid + 1;
      } else {
         hi = mid;
      }
   }

   return -1;
}

const sorted = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91];

console.log(binarySearch(sorted, 23));
console.log(binarySearch(sorted, 91));
console.log(binarySearch(sorted, 100));
