import { useRouter } from "next/router"

export default function Watch() {
  const router = useRouter()
  const { video_id } = router.query
  
  return (
    <div>
      <iframe 
        width="560" 
        height="315" 
        src={ "https://www.youtube.com/embed/" + video_id } 
        title="YouTube video player" 
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
        allowFullScreen ></iframe>
    </div>
  )
}