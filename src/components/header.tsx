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
            <nav className="flex gap-4">
                {NAV_LINKS.map((value) => <NavigationItem {...value} />)}
            </nav>
            <div className="flex gap-2 items-center">
                <button>kefnd</button>
                <button>kefnd</button>
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