import Image from "next/image"
import Link from "next/link"

interface IINALinkProps {
  url: String
}

export default function IINALink({ url: yt_video_url }: IINALinkProps) {
  // IINA Is only available on MacOS
  // if (process.platform != "darwin") {
  //   return (<></>)
  // } else {
    return (
      <Link 
        className="flex items-center gap-1 pr-3 button default-colors"
        href={"iina://weblink?url=" + yt_video_url}
      >
        <Image src="/iina-icon-512.png" alt="iina_app_icon_512x" width={32} height={32} />
        <p className="text-sm ">
          Open Video in IINA
        </p>
      </Link>
    )
  // }
}