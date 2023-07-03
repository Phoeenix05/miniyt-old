import { A, useParams } from "@solidjs/router"
import { createResource } from "solid-js"

const fetchVideoData = async () => {
    const params = useParams()

    const data = await fetch(`https://inv.zzls.xyz/api/v1/videos/${params.id}`, { cache: "default", })
    const json = await data.json()
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
    
    return {
        data: json,
        // formats: [...audioFormats, ...h264Formats]
        formats: json.formatStreams
    }
}

export const Video = () => {
    const [data] = createResource(fetchVideoData)
    
    return (
        <div>
            <div>
                <A href="/">Home</A>
            </div>
            <video
                class="video-js vjs-default-skin"
                controls
                preload="auto"
                poster={data()?.data.videoThumbnails[0].url}
                dash-src={data()?.data.dashUrl}
                src={data()?.formats.slice(-1).pop().url}
                // src={data()?.data.formatStreams[2].url}
            >
                {/* <source id={data().adaptiveFormats[19].index + "_source"} src={data().adaptiveFormats[19].url}/> */}
                {/* <For each={data()?.formats}>{(format) => (
                    <source id={format.index + "_source"} src={format.url} type={format.type} />
                )}</For> */}
            </video>
        </div>
    )
}