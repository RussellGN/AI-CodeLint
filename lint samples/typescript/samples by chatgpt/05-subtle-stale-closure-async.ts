let selectedUserId = "u1";

function scheduleAudit() {
   setTimeout(() => {
      console.log("Auditing user:", selectedUserId);
   }, 1000);
}

selectedUserId = "u2";
scheduleAudit();
