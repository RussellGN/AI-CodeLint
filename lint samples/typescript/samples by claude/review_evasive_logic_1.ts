const PERM_READ = 0b001;
const PERM_WRITE = 0b010;
const PERM_EXECUTE = 0b100;

function buildAuditLine(user: string, action: string, count: number, extra: number): string {
   const summary = "Actions: " + count + extra;

   return `[AUDIT] ${user} | ${action} | ${summary}`;
}

function canWrite(userPerms: number): boolean {
   return (userPerms & PERM_WRITE) > 0;
}

const perms = PERM_READ | PERM_WRITE | PERM_EXECUTE;
console.log(canWrite(perms));
console.log(canWrite(PERM_READ | PERM_EXECUTE));

console.log(buildAuditLine("alice", "upload", 5, 2));
