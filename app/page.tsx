import VideoCard from "@/components/VideoCard";

export default function Home() {
  return (
    <main>
      <h1>Home</h1>
      <div className="flex flex-wrap">
        <VideoCard />
        <VideoCard />
      </div>
    </main>
  )
}
