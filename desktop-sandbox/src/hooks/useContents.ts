import { useRef, useState } from "react";
import useLinter from "./useLinter";

export default function useContents() {
   const [lintResult, setLintResult] = useState<string>("");
   const [loading, setLoading] = useState(false);
   const { lintContents } = useLinter();
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

   async function onLintContents() {
      setLoading(true);
      setLintResult("");
      if (textAreaRef.current) {
         const contents = textAreaRef.current.value;
         const res = await lintContents(contents);
         setLintResult(res);
         setLoading(false);
      }
   }

   function onCloseDiagnostics() {
      setLintResult("");
   }

   return { loading, lintResult, textAreaRef, fileInputRef, openFile, onFileChange, onLintContents, onCloseDiagnostics };
}
