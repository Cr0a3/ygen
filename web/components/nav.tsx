"use client";

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
import {
    NavigationMenu,
    NavigationMenuContent,
    NavigationMenuIndicator,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenuTrigger,
    navigationMenuTriggerStyle,
    NavigationMenuViewport,
} from "@/components/ui/navigation-menu"
import React from "react";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { useState } from "react";

export const NavBar = () => {
    let [isOpen, setIsOpen] = useState(false);

    const toggleMenu = () => {
        setIsOpen(!isOpen);
    };

    return (
        <div className="top-0 left-0 border-0">
            <nav className="relative">
                <div className="flex items-center justify-between p-4 md:p-6">
                    <button
                        onClick={toggleMenu}
                        className="block md:hidden focus:outline-none"
                    >
                        <svg
                            className="w-6 h-6"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth="2"
                                d="M4 6h16M4 12h16m-7 6h7"
                            />
                        </svg>
                    </button>

                    <NavigationMenu
                        className={`${isOpen ? "block" : "hidden"
                            } absolute top-16 left-0 w-full bg-white shadow-md md:static md:block md:w-auto`}
                    >
                        <NavigationMenuList className="flex flex-col md:flex-row">
                            <NavigationMenuItem>
                                <Link href="/" legacyBehavior passHref>
                                    <NavigationMenuLink className={navigationMenuTriggerStyle()}>
                                        Home
                                    </NavigationMenuLink>
                                </Link>
                            </NavigationMenuItem>

                            <NavigationMenuItem>
                                <Link href="/learn" legacyBehavior passHref>
                                    <NavigationMenuLink className={navigationMenuTriggerStyle()}>
                                        Learn
                                    </NavigationMenuLink>
                                </Link>
                            </NavigationMenuItem>

                            <NavigationMenuItem>
                                <Link href="/docs" legacyBehavior passHref>
                                    <NavigationMenuLink className={navigationMenuTriggerStyle()}>
                                        Docs
                                    </NavigationMenuLink>
                                </Link>
                            </NavigationMenuItem>
                        </NavigationMenuList>
                    </NavigationMenu>
                </div>
            </nav>
        </div>
    );
};

export const PageHeader = ({ page, small }: { page: string, small: boolean }) => {
    return (
        <div>
            {/*<div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
                <span className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
                    <pre>Developed by </pre>
                    <HoverCard>
                        <HoverCardTrigger asChild>
                            <a href="https://github.com/Cr0a3" className="text-blue-500 hover:text-blue-700">@Cr0a3</a>
                        </HoverCardTrigger>
                        <HoverCardContent className="w-80">
                            <div className="flex justify-between space-x-2">
                                <Avatar>
                                    <AvatarImage src="https://avatars.githubusercontent.com/u/127748753?v=4" />
                                    <AvatarFallback>Cr0a3</AvatarFallback>
                                </Avatar>
                                <div className="space-y-1">
                                    <h4 className="text-sm font-semibold">@Cr0a3</h4>
                                    <p className="text-sm">
                                        I like compiler development
                                    </p>
                                    <div className="flex items-center pt-2">
                                        <span className="text-xs text-muted-foreground">
                                            Joined March 2023
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </HoverCardContent>
                    </HoverCard>
                </span>
            </div>*/}

            <div className="p-16 lg:p-4 relative z-[-1] flex place-items-center before:absolute before:h-[300px] before:w-full before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-full after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#0141ff] after:dark:opacity-40 sm:before:w-[480px] sm:after:w-[240px] before:lg:h-[360px]">
                <div className={`${small ? 'text-7xl' : 'text-9xl'} font-bold`}>
                    {page}
                </div>
            </div>
        </div>
    );
};