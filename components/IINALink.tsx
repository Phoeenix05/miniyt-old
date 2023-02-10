import Image from "next/image";
import Link from "next/link";

interface IINALinkProps {
  url: String
  // children: [React.ReactNode]
}

export default function IINALink({ url: yt_video_url }: IINALinkProps) {
  return (
    <Link 
      // className="flex items-center w-40 gap-1 p-1 transition-all duration-100 border-2 rounded-md bg-dark-800 hover:bg-dark-700 border-dark-600 hover:border-dark-500 text-dark-300 hover:text-white"
      className="flex items-center gap-1 pr-3 max-w-fit button default-colors"
      href={"iina://weblink?url=" + yt_video_url}
    >
      <Image src="/iina-icon-512.png" alt="iina_app_icon_512x" width={32} height={32} />
      <p className="text-sm ">
        Open Video in IINA
      </p>
    </Link>
  )
}