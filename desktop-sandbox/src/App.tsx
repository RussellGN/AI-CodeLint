import useContents from "./hooks/useContents";

export default function App() {
   const { fileInputRef, onFileChange, openFile, textAreaRef } = useContents();

   return (
      <div className="h-screen flex flex-col p-5">
         <div className="flex  gap-5 items-center mb-5">
            <h1 className="mr-auto">AI CodeLint Desktop Sandbox</h1>

            <input ref={fileInputRef} onChange={onFileChange} type="file" className="hidden" />
            <button onClick={openFile} className="">
               Open file
            </button>
            <button className="">Lint Contents</button>
         </div>

         <textarea ref={textAreaRef} className="grow outline-0 rounded-sm p-5 border-2 border-DARK shadow-2xl  bg-LIGHT/5   text-sm font-mono" name="text" rows={10}></textarea>
      </div>
   );
}
