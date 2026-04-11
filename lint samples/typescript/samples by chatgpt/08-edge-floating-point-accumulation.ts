const prices = [0.1, 0.2, 0.3];
const total = prices.reduce((sum, price) => sum + price, 0);

export const isExact = total === 0.6;
