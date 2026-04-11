function coerce<T>(value: string): T {
   return JSON.parse(value) as unknown as T;
}

interface UserRecord {
   id: number;
   email: string;
   isAdmin: boolean;
}

const user = coerce<UserRecord>('{"id": "not-a-number", "email": 42}');

console.log(user.id.toFixed(2));
console.log(user.isAdmin === true);
