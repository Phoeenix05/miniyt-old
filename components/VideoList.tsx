interface VideoListProps {
  children: Array<React.ReactNode>
}

export default function VideoList({ children }: VideoListProps) {
  return (
    <div>
      { children }
    </div>
  )
}