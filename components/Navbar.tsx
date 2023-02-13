import { HomeIcon, UsersIcon } from "@heroicons/react/24/solid"
import NavLink from "./NavLink"

export default function Navbar() {
  return (
    // <nav className="w-screen h-12 bg-dark-900">
    // {/* <nav className="w-12 h-screen bg-dark-900"> */}
    //   <div id="content" className="flex items-center justify-between h-full lg:mx-24 md:mx-16 sm:mx-8"> 
    //     <Link href="/" className="px-4 py-2 text-2xl font-bold">
    //       MiniYT
    //     </Link>
    //     <div id="navigation">
    //       <Link href="/follows" className="px-4 py-2 ">
    //         Follows
    //       </Link>
    //     </div>
    //   </div>
    // </nav>
    <nav className="fixed top-0 left-0 md:w-48 w-[4.2rem] h-screen overflow-x-hidden z-[99] bg-dark-800 border-r-2 border-dark-700 pt-6">
      {/* <HomeIcon />
      <Link href="/follows" className="px-4 py-2 ">
        <UsersIcon />
        Follows
      </Link> */}
      <NavLink href="/" title="Home">
        <HomeIcon className="icon" />
      </NavLink>
      <NavLink href="/follows" title="Follows">
        <UsersIcon className="icon" />
      </NavLink>
    </nav>
  )
}