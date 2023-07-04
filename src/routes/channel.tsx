import { A, useParams } from "@solidjs/router"
import { For, Show, createResource } from "solid-js"
import { VideoCard } from "../components/video/video-card"
import { construct_url } from "../util/url"

const fetchChannelData = async () => {
    const params = useParams()

    const data = await fetch(construct_url(`channels/${params.id}`), { cache: "force-cache" })
    const json = await data.json()
    return json
}

export const Channel = () => {
    const [data] = createResource(fetchChannelData)
    
    return (
        <>
            <A href="/">Home</A>
            <Show when={!data.loading} fallback={<>Loading...</>}>
                <img src={data().authorBanners[0].url} />
                <div class="grid gap-4 xl:grid-cols-5 lg:grid-cols-4 md:grid-cols-3 grid-cols-2 content-stretch w-full">
                    <For each={data().latestVideos}>
                        {(video, _i) => (
                            <VideoCard data={video} />
                        )}
                    </For>
                </div>
                {/* <pre>{JSON.stringify(data(), null, 2)}</pre> */}
            </Show>
        </>
    )
}