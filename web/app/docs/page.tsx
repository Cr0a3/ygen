import { NavBar, PageHeader } from "@/components/nav";
import { InfoBox } from "@/components/infoBox";

export default function Page() {
    return (
    <>
    <div className="top-0 h-full w-full mb-0 pb-0">

      <div className="max-w-full top-0 m-4 flex w-fit">
        <NavBar />
      </div>
      <main className="flex min-h-screen flex-col items-center">

        <div className="pb-16 lg:pb-32">
            <PageHeader page={"ygen - Docs"} small={true} />
        </div>

        <div className="text-left inset-x-1/3 w-2/3">
            <h3>Ygens documentation</h3>
            <br />
            <p>
            Ygen currently doesn't host the documentation here on its site.
            It uses <code>rustdoc</code> for automatic documentation deployment.
            Because of that you can find the link to the documentation below
            </p>
            <br />

            <div className="grid text-center items-center lg:mb-0 lg:w-full lg:max-w-5xl lg:grid-cols-1 lg:text-left">
                <InfoBox name={"Documentation"} descr={"Ygens documentation"} link={"https://cr0a3.github.io/ygen/ygen/"} />
            </div>
        </div>
      </main>

    </div>
    </>
    );
}