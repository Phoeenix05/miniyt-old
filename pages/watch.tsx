// import Link from "next/link"
import { useRouter } from "next/router"
// import { useEffect, useState } from "react"

export default function Watch() {
  const router = useRouter()
  const { video_id } = router.query
  
  // const [visible, setVisible] = useState(false)
  // const [id, setId] = useState("lIV8qIKxVkk")

  return (
    <div className="mt-10 ml-48">
      {/* {visible ? <iframe 
        width="560" 
        height="315" 
        src={ "https://www.youtube.com/embed/" + video_id } 
        title="YouTube video player" 
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
        allowFullScreen ></iframe> : <></>}
      <button onClick={() => setVisible(true)} className="cursor-pointer">Show</button> */}
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