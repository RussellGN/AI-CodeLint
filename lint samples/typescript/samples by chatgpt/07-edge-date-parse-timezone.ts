const cutoff = new Date("2026-04-11");
const lastSeen = new Date("2026-04-10T23:30:00-01:00");

export const isExpired = lastSeen < cutoff;
