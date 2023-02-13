import Link from "next/link"

interface NavLinkProps {
  href: string
  title: String
  children?: React.ReactNode
}

export default function NavLink({ href, title, children }: NavLinkProps) {
  return (
    <Link href={href} className="flex gap-4 md:p-2 md:px-4 navlink">
      { children }
      <h2 className="invisible font-semibold tracking-wide md:visible">{ title }</h2>
    </Link>
  )
}