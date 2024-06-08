"use client";

import Editor from "@monaco-editor/react";
export default async function IDE() {
    const exports = await import("../../pkg/ai");
    const { run_source: run_source } = exports;


    const handleSubmit = async (
        event: React.FormEvent<HTMLFormElement>
    ) => {
        event.preventDefault();

        let code = event.target[0].value;
        let output = run_source(code);

        console.log(output);
    }

    return (
        <div className="flex justify-center items-start pt-10 h-screen w-full">
            <div className="w-full max-w-4xl p-4 border">
                <form action="#" onSubmit={handleSubmit}>
                    <div className="">
                        <label htmlFor="comment" className="sr-only">
                            Add your code
                        </label>
                        <Editor
                            height="50vh"
                            defaultLanguage="javascript"
                            defaultValue="let x = 1;
                                          let y = 2;
                                        print(x + y);"
                        />
                    </div>
                    <div className="flex justify-between pt-2">
                        <div className="flex items-center space-x-5"></div>
                        <div className="flex-shrink-0">
                            <button
                                type="submit"
                                className="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-500"
                            >
                                Run
                            </button>
                        </div>
                    </div>
                </form>
            </div>
        </div>
    );
}