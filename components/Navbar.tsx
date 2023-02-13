import { HomeIcon, UsersIcon } from "@heroicons/react/24/solid"
import NavLink from "./NavLink"

export default function Navbar() {
  return (
    <nav className="fixed top-0 left-0 md:w-48 w-[4.2rem] h-screen overflow-x-hidden z-[99] bg-dark-800 border-r-2 border-dark-700 pt-6">
      <NavLink href="/" title="Home">
        <HomeIcon className="icon" />
      </NavLink>
      <NavLink href="/follows" title="Follows">
        <UsersIcon className="icon" />
      </NavLink>
    </nav>
  )
}