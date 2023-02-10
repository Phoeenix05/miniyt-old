import VideoCard from "@/components/VideoCard";
import VideoList from "@/components/VideoList";

export default function Home() {
  return (
    <main>
      <h1>Home</h1>
      {/* <div className="flex flex-wrap justify-center">
        <VideoCard />
        <VideoCard />
      </div> */}
      <VideoList>
        <VideoCard />
        <VideoCard />
      </VideoList>
    </main>
  )
}
