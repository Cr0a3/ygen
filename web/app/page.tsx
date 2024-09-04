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
import { InfoBox } from "@/components/infoBox";

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
