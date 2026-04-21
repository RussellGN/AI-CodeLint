import { useRef } from "react";

export default function useContents() {
   const textAreaRef = useRef<HTMLTextAreaElement>(null);
   const fileInputRef = useRef<HTMLInputElement>(null);

   const openFile = () => {
      if (fileInputRef.current) {
         fileInputRef.current.click();
      }
   };

   function onFileChange(e: React.ChangeEvent<HTMLInputElement>) {
      const file = e.target.files?.[0];
      if (file) {
         const reader = new FileReader();
         reader.onload = (event) => {
            if (textAreaRef.current) {
               textAreaRef.current.value = event.target?.result as string;
            }
         };
         reader.readAsText(file);
      }
   }

   return { textAreaRef, fileInputRef, openFile, onFileChange };
}
