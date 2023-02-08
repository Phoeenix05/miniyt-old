import Link from "next/link";

interface IINALinkProps {
  youtube_video_url: String
  children: [React.ReactNode]
}

export default function IINALink({ youtube_video_url, children }: IINALinkProps) {
  return (
    <Link href={"iina://weblink?url=" + youtube_video_url}>
      { children }
    </Link>
  )
}