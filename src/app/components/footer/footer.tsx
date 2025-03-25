import Image from "next/image";
import Link from "next/link";
import LinkList from "./footer-link-list";
import {
  BlockBatchLogo,
  Twitter,
  Github,
  LinkedIn,
} from "../../../../public/assets";

const footerLinks = {
  product: [
    { label: "Features", href: "/features" },
    { label: "Pricing", href: "/pricing" },
    { label: "API", href: "/api" },
  ],
  company: [
    { label: "About", href: "/about" },
    { label: "Blog", href: "/blog" },
    { label: "Careers", href: "/careers" },
  ],
  resources: [
    { label: "Documentation", href: "/docs" },
    { label: "Support", href: "/support" },
    { label: "Guides", href: "/guides" },
  ],
  legal: [
    { label: "Privacy", href: "/privacy" },
    { label: "Terms", href: "/terms" },
    { label: "Security", href: "/security" },
  ],
};

export default function Footer() {
  return (
    <footer className="bg-white border-t border-gray-200">
      <div className="max-w-7xl mx-auto px-4 py-12 sm:px-6 lg:px-8">
        <div className="max-w-6xl mx-auto flex flex-col lg:flex-row justify-between items-start gap-12">
          {/* Logo and tagline section */}
          <div className="flex flex-col items-start">
            <Image
              src={BlockBatchLogo}
              alt="BlockBatch Logo"
              width={142}
              height={40}
              className="h-8 w-auto"
            />
            <p className="mt-2 text-gray-600 lg:whitespace-nowrap">
              Automating batch payments with blockchain technology.
            </p>
          </div>

          {/* Links section */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            <LinkList title="Product" links={footerLinks.product} />
            <LinkList title="Company" links={footerLinks.company} />
            <LinkList title="Resources" links={footerLinks.resources} />
            <LinkList title="Legal" links={footerLinks.legal} />
          </div>
        </div>

        {/* Bottom section with copyright and social links */}
        <div className="flex md:mx-5 flex-col sm:flex-row justify-between items-center pt-8 border-t border-gray-200 mt-12">
          <p className="text-gray-500 text-sm">
            © {new Date().getFullYear()} BlockBatch. All rights reserved.
          </p>

          <div className="flex gap-6 mt-4 sm:mt-0">
            <Link
              href="#"
              aria-label="Twitter"
              className="text-gray-400 hover:text-gray-500"
            >
              <Twitter className="h-6 w-6" />
            </Link>
            <Link
              href="#"
              aria-label="GitHub"
              className="text-gray-400 hover:text-gray-500"
            >
              <Github className="h-6 w-6" />
            </Link>
            <Link
              href="#"
              aria-label="LinkedIn"
              className="text-gray-400 hover:text-gray-500"
            >
              <LinkedIn className="h-6 w-6" />
            </Link>
          </div>
        </div>
      </div>
    </footer>
  );
}
