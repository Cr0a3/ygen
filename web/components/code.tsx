'use client';

import { Button } from "@/components/ui/button";
import { Copy } from "lucide-react";

export const CodeBlock = ({code, lang}:{code: string, lang: string}) => {
    const copy = () => {
        navigator.clipboard.writeText(code);
    };

    return (<div className="relative bg-gray-200 w-fit rounded-lg">
        <div className="absoulte top-0 left-0 p-3">
            <p className="font-semibold text-gray-500">{lang}</p>
        </div>
        <div className="absolute top-0 right-0 p-3">
            <Button onClick={() => copy()}><Copy /></Button>
        </div>
        <pre className="p-4">
            <code>
{code}
            </code>
        </pre>
    </div>);
}