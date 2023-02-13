interface WatchProps {
  video_id: string
}

export default function Watch({ video_id }: WatchProps) {
  return (
    <div>
      <iframe 
        width="560" 
        height="315" 
        src={ "https://www.youtube.com/embed/" + video_id } 
        title="YouTube video player" 
        // frameBorder="0" 
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
        allowFullScreen ></iframe>
      {/* <iframe width="560" height="315" src="https://www.youtube.com/embed/lIV8qIKxVkk" title="YouTube video player" frameBorder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowFullScreen></iframe> */}
    </div>
  )
}