import Link from "next/link";
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card"
import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from "@/components/ui/avatar"
import { NavBar, PageHeader } from "@/components/nav";

export const InfoBox = ({ name, descr, link }: { name: string, descr: string, link: string }) => {
  return (
    <Link
      href={link}
      className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
      rel="noopener noreferrer"
    >
      <h2 className="mb-3 text-2xl font-semibold">
        {name}
        <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
          -&gt;
        </span>
      </h2>
      <p className="m-0 max-w-[30ch] text-sm opacity-50">
        {descr}
      </p>
    </Link>
  );
}

export default function Home() {
  return (<>
    <div className="max-w-full max-h-fit min-h-fit h-full">

      <div className="max-w-full top-0 m-4 flex w-fit">
        <NavBar />
      </div>
      <main className="flex min-h-screen flex-col items-center justify-between p-24">

        <PageHeader page={"ygen"} />

        <div className="mb-32 grid text-center items-center lg:mb-0 lg:w-full lg:max-w-5xl lg:grid-cols-3 lg:text-left">
          <InfoBox name={"Docs"} descr={"Find information about ygens features"} link={"/docs"} />
          <InfoBox name={"Examples"} descr={"Find information about ygens features"} link={"/examples"} />
          <InfoBox name={"Learn"} descr={"Learn how to build compilers with ygen"} link={"/learn"} />
        </div>
      </main>

    </div>
  </>);
}
