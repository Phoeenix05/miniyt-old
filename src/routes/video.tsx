import { A, useParams } from "@solidjs/router"
import { createResource } from "solid-js"
import { VideoResult } from "../util/invidious/video"

const fetchVideoData = async () => {
    const params = useParams()

    const data = await fetch(`https://inv.zzls.xyz/api/v1/videos/${params.id}`, { cache: "default", })
    const json: VideoResult = await data.json()
    return json
    // console.log(json)

    // const adaptiveFormats = json.adaptiveFormats
    // const audioFormats: any = []
    // const h264Formats: any = []
    // const av1Formats: any = []

    // adaptiveFormats.forEach((format: any) => {
    //     const mimeType = format.type

    //     if (mimeType.startsWith('audio/mp4')) {
    //         audioFormats.push(format)
    //     } else if (mimeType.startsWith('video/mp4; codecs="av01') ) {
    //         av1Formats.push(format)
    //     } else if (mimeType.startsWith('video/mp4; codecs="avc')) {
    //         h264Formats.push(format)
    //     }
    // })

    // return [...audioFormats, ...h264Formats]
}

export const Video = () => {
    const [data] = createResource(fetchVideoData)
    
    return (
        <>
            <div>
                <A href="/">Home</A>
            </div>
            <video
                class="video-js vjs-default-skin"
                controls
                preload="auto"
                poster={data()?.videoThumbnails[0].url}
                src={data()?.formatStreams.at(-1)?.url}
            >
            </video>
        </>
    )
}