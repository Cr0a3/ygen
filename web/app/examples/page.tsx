import { NavBar, PageHeader } from "@/components/nav";
import { InfoBox } from "@/components/infoBox";

export default function Page() {
    return (
    <>
    <div className="top-0 h-full w-full">

      <div className="max-w-full top-0 m-4 flex w-fit">
        <NavBar />
      </div>
      <main className="flex min-h-screen flex-col items-center">

        <div className="pb-16 lg:pb-32">
            <PageHeader page={"ygen - Examples"} small={true} />
        </div>

        <div className="text-left inset-x-1/3 w-2/3">
            <h3>Ygens examples</h3>
            <br />
            <p>
            Ygen has some examples to show you how to use it in real world applications.
            The examples can be found on github
            </p>
            <br />

            <div className="grid text-center items-center lg:mb-0 lg:w-full lg:max-w-5xl lg:grid-cols-3 lg:text-left">
                <InfoBox name={"Simplelang"} descr={"A simple example programming language"} link={"https://github.com/Cr0a3/ygen/tree/main/tools/simplelang"} />
                <InfoBox name={"Github Examples"} descr={"Other examples"} link={"https://github.com/Cr0a3/ygen/tree/main/examples/"} />
                <InfoBox name={"Learn"} descr={"Learn how to build compilers with ygen"} link={"/learn"} />
            </div>
        </div>
      </main>

    </div>
    </>
    );
}