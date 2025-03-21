import {
    Sheet,
    SheetContent,
    SheetTrigger
} from "@/components/ui/sheet";
import { Menu } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import logo from '../../public/logo.png';

const NAV_LINKS = [{
    text: 'Features',
    url: '#features'
}, {
    text: 'Comparison',
    url: '#comparison'
}, {
    text: 'Explore',
    url: '#explore'
}, {
    text: 'Dashboard',
    url: '#dashboard'
}]
export function Header() {
    return (
        <div className="border-b-2 border-border">
            <header className="mx-auto container px-4 md:px-12 py-3  flex justify-between items-center">
                <Image src={logo} width={73} alt="BlockBatch" height={40} />
                <nav className="hidden md:flex gap-6">
                    {NAV_LINKS.map((value, index) => <NavigationItem key={`${index}_${Math.random()}`} {...value} />)}
                </nav>
                <div className=" hidden md:flex gap-2.5 items-center">
                    <button className="font-medium text-sm">Login</button>
                    <button className="px-4 py-2.5 bg-primary text-white rounded-md font-medium text-sm hover:opacity-95 hover:scale-102">Get Started</button>
                </div>

                {/* This is a sheet for Mobile Navigation */}
                <Sheet>
                    <SheetTrigger className="block md:hidden"><Menu /></SheetTrigger>
                    <SheetContent className="divide-y-2">
                        <nav className="flex flex-col gap-6 mt-8 pb-4">
                            {NAV_LINKS.map((value, index) => <NavigationItem key={`${index}_${Math.random()}`} {...value} />)}
                        </nav>
                        <div className=" flex flex-col gap-2.5 mt-2">
                            <button className="px-4 py-2.5 bg-primary text-white rounded-md font-medium text-sm">Get Started</button>
                            <button className="font-medium text-sm border rounded-sm px-4 py-2.5">Login</button>
                        </div>
                    </SheetContent>
                </Sheet>
            </header>
        </div>
    )
}

function NavigationItem({ text, url }: {
    text: string;
    url: string
}) {
    return (
        <Link href={url}>{text}</Link>
    )
}