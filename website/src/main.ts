import "./style.css";

const COPY_SUCCESS_TEXT = "Copied";
const COPY_FAILURE_TEXT = "Failed";

const fallbackCopyText = (text: string): boolean => {
   const textArea = document.createElement("textarea");
   textArea.value = text;
   textArea.setAttribute("readonly", "");
   textArea.style.position = "fixed";
   textArea.style.opacity = "0";
   textArea.style.pointerEvents = "none";
   document.body.appendChild(textArea);
   textArea.focus();
   textArea.select();

   let copied = false;
   try {
      copied = document.execCommand("copy");
   } finally {
      document.body.removeChild(textArea);
   }

   return copied;
};

const copyText = async (text: string): Promise<boolean> => {
   if (navigator.clipboard?.writeText) {
      try {
         await navigator.clipboard.writeText(text);
         return true;
      } catch {
         // Fall back to execCommand copy below.
      }
   }

   return fallbackCopyText(text);
};

const setTemporaryButtonState = (button: HTMLButtonElement, label: string) => {
   const originalLabel = button.dataset.originalLabel ?? button.textContent ?? "Copy";
   button.dataset.originalLabel = originalLabel;

   button.textContent = label;
   button.disabled = true;

   window.setTimeout(() => {
      button.textContent = originalLabel;
      button.disabled = false;
   }, 1400);
};

const initCopyButtons = () => {
   const copyButtons = document.querySelectorAll<HTMLButtonElement>(".copy-script-btn");

   copyButtons.forEach((button) => {
      button.addEventListener("click", async () => {
         const targetId = button.dataset.copyTarget;
         if (!targetId) {
            return;
         }

         const codeBlock = document.getElementById(targetId);
         const command = codeBlock?.textContent?.trim();
         if (!command) {
            return;
         }

         const copied = await copyText(command);
         setTemporaryButtonState(button, copied ? COPY_SUCCESS_TEXT : COPY_FAILURE_TEXT);
      });
   });
};

initCopyButtons();
