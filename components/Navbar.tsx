import Link from "next/link";

export default function Navbar() {
  return (
    <nav className="w-screen h-12 bg-neutral-900">
      <div id="content" className="flex items-center justify-between h-full lg:mx-24 md:mx-16 sm:mx-8">
        <Link href="/" className="px-4 py-2 text-2xl font-bold">MiniYT</Link>
        <div id="navigation">
          <Link href="/follows" className="px-4 py-2 font-semibold ">Follows</Link>
        </div>
      </div>
    </nav>
  )
}