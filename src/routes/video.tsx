import { A, useParams } from "@solidjs/router"
import { Show, createResource } from "solid-js"

const fetchVideoData = async () => {
    const params = useParams()

    const data = await fetch(`https://inv.zzls.xyz/api/v1/videos/${params.id}`, { cache: "default", })
    const json = await data.json()
    console.log(json)
    
    return json
}

export const Video = () => {
    const [data] = createResource(fetchVideoData)
    
    return (
        <div>
            <div>
                <A href="/">Home</A>
            </div>
            <Show when={!data.loading} fallback={<>Loading...</>}>
                <video
                    controls
                    preload="auto"
                    src={data().formatStreams[2].url}
                >
                </video>
            </Show>
        </div>
    )
}