// import VideoCard from "./VideoCard"

interface VideoListProps {
  // videos: [typeof VideoCard]
  // videos: [JSX.Element<VideoCard>]
  // videos: [React.ReactNode]
  children: Array<React.ReactNode>
}

export default function VideoList({ children }: VideoListProps) {
  return (
    <div>
      { children }
    </div>
  )
}