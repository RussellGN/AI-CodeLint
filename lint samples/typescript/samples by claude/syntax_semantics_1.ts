interface Displayable {
   id: number;
   getLabel(): string;
}

class Product implements Displayable {
   id: number;
   private name: string | null;

   constructor(id: number, name: string | null) {
      this.id = id;
      this.name = name;
   }

   getLabel(): string | null {
      return this.name;
   }
}

function renderItem(item: Displayable): void {
   console.log(`[${item.id}] ${item.getLabel()}`);
}

const p = new Product(1, null);
renderItem(p);
