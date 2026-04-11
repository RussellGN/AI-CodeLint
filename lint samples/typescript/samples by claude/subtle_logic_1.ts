interface CartItem {
   name: string;
   price: number;
}

interface Cart {
   items: CartItem[];
   total: number;
}

function applyDiscount(cart: Cart, discountPct: number): Cart {
   const discounted = { ...cart };

   discounted.items.forEach((item) => {
      item.price = item.price * (1 - discountPct / 100);
   });

   discounted.total = discounted.items.reduce((sum, i) => sum + i.price, 0);
   return discounted;
}

const myCart: Cart = {
   items: [
      { name: "Widget", price: 100 },
      { name: "Gadget", price: 200 },
   ],
   total: 300,
};

const saleCart = applyDiscount(myCart, 10);

console.log("Original total:", myCart.total);
console.log("Original item:", myCart.items[0].price);
console.log("Sale total:", saleCart.total);
