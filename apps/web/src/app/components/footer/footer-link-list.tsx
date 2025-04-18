import { FC } from "react";

interface LinkItem {
  label: string;
  href: string;
}

interface LinkListProps {
  title: string;
  links: LinkItem[];
}

const LinkList: FC<LinkListProps> = ({ title, links }) => {
  return (
    <div className="flex flex-col gap-4">
      <h3 className="font-semibold text-gray-800">{title}</h3>
      <ul className="flex flex-col gap-2">
        {links.map((link) => (
          <li key={link.label}>
            <a
              href={link.href}
              className="text-gray-600 hover:text-gray-900 transition-colors"
              aria-label={link.label}
            >
              {link.label}
            </a>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default LinkList;
