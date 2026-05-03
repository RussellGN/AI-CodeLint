function getValues(obj: Record<string, number>, keys: string[]): number[] {
   return keys.map((k) => obj[k] || 0);
}
