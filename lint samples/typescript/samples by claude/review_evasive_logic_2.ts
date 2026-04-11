interface Task {
   id: number;
   payload: string;
}

async function processItem(task: Task, index: number): Promise<void> {
   await new Promise((res) => setTimeout(res, 10));
   console.log(`Processed task #${index} → id=${task.id}`);
}

async function runBatch(tasks: Task[]): Promise<void> {
   const promises: Promise<void>[] = [];

   for (var i = 0; i < tasks.length; i++) {
      const p = (async () => {
         await processItem(tasks[i], i);
      })();
      promises.push(p);
   }

   await Promise.all(promises);
}

runBatch([
   { id: 101, payload: "alpha" },
   { id: 102, payload: "beta" },
   { id: 103, payload: "gamma" },
]);
