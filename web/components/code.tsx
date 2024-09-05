'use client';

import { Button } from "@/components/ui/button";
import { Copy } from "lucide-react";
import { useState } from "react";

export const CodeBlock = ({ code, lang }: { code: string, lang: string }) => {
    const [clicked, setClicked] = useState(false);
    const [animate, setAnimate] = useState(false);

    const copy = () => {
        navigator.clipboard.writeText(code);

        setClicked(!clicked);
        setAnimate(true);
        setTimeout(() => {
            setAnimate(false);
        }, 500);
    };

    return (<div className="relative bg-gray-200 w-fit rounded-lg">
        <div className="absoulte top-0 left-0 p-3">
            <p className="font-semibold text-gray-500">{lang}</p>
        </div>
        <div className="absolute top-0 right-0 p-3">
            <Button className={`${animate ? "bg-green-500 hover:bg-gren-600 animate-ping" : "bg-gray-500 hover:bg-gray-600"} `} onClick={() => copy()}><Copy /></Button>
        </div>
        <pre className="p-4">
            <code>
                {code}
            </code>
        </pre>
    </div>);
}