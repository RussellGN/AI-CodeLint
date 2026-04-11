async function loadUsernames(ids: number[]) {
   const usernames = ids.map(async (id) => {
      const response = await fetch(`/api/users/${id}`);
      const user = await response.json();
      return user.name as string;
   });

   return usernames;
}
