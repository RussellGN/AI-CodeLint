const retries: number = "3";

export function waitForRetry(): number {
   return retries + 1;
}
