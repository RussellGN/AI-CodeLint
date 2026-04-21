import useContents from "./hooks/useContents";

export default function App() {
   const { loading, fileInputRef, onFileChange, onLintContents, openFile, lintResult, textAreaRef, onCloseDiagnostics } = useContents();

   return (
      <div className="h-screen flex flex-col p-5">
         <div className="flex  gap-5 items-center mb-5">
            <h1 className="mr-auto">AI CodeLint Desktop Sandbox</h1>

            <input disabled={loading} ref={fileInputRef} onChange={onFileChange} type="file" className="hidden" />
            <button disabled={loading} onClick={openFile} className="">
               Open file
            </button>
            <button disabled={loading} onClick={onLintContents} className="">
               Lint Contents
            </button>
         </div>

         <div className="grow flex relative">
            {loading || lintResult ? (
               <div className="bg-DARK/90 flex items-start gap-4 text-red-300 rounded-bl-sm  shadow-2xl   absolute top-0 right-0 w-fit p-5  max-w-2/3">
                  {loading ? (
                     <div className="flex items-center gap-2 text-LIGHT">
                        {" "}
                        <div className="animate-spin size-6  border-2  rounded-full border-r-transparent  border-LIGHT" />
                        linting...
                     </div>
                  ) : (
                     lintResult
                  )}

                  <button disabled={loading} onClick={onCloseDiagnostics}>
                     x
                  </button>
               </div>
            ) : (
               ""
            )}
            <textarea
               disabled={loading}
               ref={textAreaRef}
               className="grow outline-0 rounded-sm p-5 border-2 border-DARK disabled:cursor-not-allowed disabled:opacity-50 shadow-2xl  bg-LIGHT/5   text-sm font-mono"
               name="text"
               rows={10}
               placeholder="Type..."
            ></textarea>
         </div>
      </div>
   );
}
