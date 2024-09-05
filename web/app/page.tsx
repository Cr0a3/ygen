import { NavBar, PageHeader } from "@/components/nav";
import { InfoBox } from "@/components/infoBox";

export default function Home() {
  return (<>
    <div className="top-0 h-full w-full">

      <div className="max-w-full top-0 m-4 flex w-fit">
        <NavBar />
      </div>
      <main className="flex min-h-screen flex-col items-center justify-between p-24">

        <PageHeader page={"ygen"} small={false} />

        <div className="mb-32 grid text-center items-center lg:mb-0 lg:w-full lg:max-w-5xl lg:grid-cols-3 lg:text-left">
          <InfoBox name={"Docs"} descr={"Find information about ygens features"} link={"/docs"} />
          <InfoBox name={"Examples"} descr={"Find information about ygens features"} link={"/examples"} />
          <InfoBox name={"Learn"} descr={"Learn how to build compilers with ygen"} link={"/learn"} />
        </div>
      </main>

    </div>
  </>);
}
