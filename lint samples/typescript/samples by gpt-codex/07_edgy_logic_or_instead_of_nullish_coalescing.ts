function applyDiscount(price: number, discountPercent?: number): number {
   const discount = discountPercent || 10;
   return price * (1 - discount / 100);
}
