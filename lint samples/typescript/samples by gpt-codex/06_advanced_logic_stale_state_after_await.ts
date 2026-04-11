let cacheVersion = 0;
const cache = new Map<string, string>();

async function refreshIfNeeded(key: string, fetcher: () => Promise<string>) {
   const startVersion = cacheVersion;
   const value = await fetcher();

   if (startVersion === cacheVersion) {
      cache.set(key, value);
   }

   cacheVersion++;
}
