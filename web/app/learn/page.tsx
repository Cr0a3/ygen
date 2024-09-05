import { NavBar, PageHeader } from "@/components/nav";
import { InfoBox } from "@/components/infoBox";
import { CodeBlock } from "@/components/code";

export default function Page() {
    return (
    <>
    <div className="top-0 h-full w-full">

      <div className="max-w-full top-0 m-4 flex w-fit">
        <NavBar />
      </div>
      <main className="flex min-h-screen flex-col items-center">

        <div className="pb-16 lg:pb-32">
            <PageHeader page={"ygen - Learn"} small={true} />
        </div>

        <div className="text-left inset-x-1/3 w-2/3">
            <h3>Learning ygen</h3>
            <br />

            <span id="article">
                <p className="pb-5">
                Welcome to the ygen-book! Here you will learn how to build a simple compiler with ygen.
                I won't cover how to implement the parser, lexer, semnatic analayis here but you can refere to our examples.
                </p>

                <div className="grid text-center items-center lg:mb-0 lg:w-full lg:max-w-5xl lg:grid-cols-1 lg:text-left pb-5">
                    <InfoBox name={"How to frontend"} descr={"Simple example code on how to implement an parser, lexer"} link={"https://github.com/Cr0a3/ygen/tree/main/tools/simplelang"} />
                </div>

                
                <h3>Our ast (Abstract Syntax Tree)</h3>
                <br />

                <p className="pb-5">
                Every language has a ast which represents the <code>expressions</code> and <code>statements</code>.
                For our language we will have following statements:
                <p className="pb-5" />
<CodeBlock lang={"Rust"}
code={`#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {  
    FnStmt(FnStmt),
}`}/>
                </p>
            </span>
        </div>
      </main>

    </div>
    </>
    );
}