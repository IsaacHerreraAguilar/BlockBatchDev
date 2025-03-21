import Image from "next/image";
import logo from '../../public/logo.png'
import Link from "next/link";

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
        <div className="mx-auto container px-12 py-3  flex justify-between items-center">
            <Image src={logo} width={73} alt="BlockBatch" height={40} />
            <nav className="flex gap-6">
                {NAV_LINKS.map((value, index) => <NavigationItem key={`${index}_${Math.random()}`} {...value} />)}
            </nav>
            <div className="flex gap-2.5 items-center">
                <button>Login</button>
                <button className="px-4 py-2.5 bg-primary text-white rounded-md ">Get Started</button>
            </div>
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