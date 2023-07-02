import { A, useParams } from "@solidjs/router"
import { For, Show, createResource } from "solid-js"
import { VideoCard } from "../components/video-card"

const fetchSearchData = async () => {
    const params = useParams()

    const data = await fetch(`https://invidious.tiekoetter.com/api/v1/search/?q=${params.q}&page=1&sort_by=relevance&date=&duration=&type=all`, {
        cache: "default",
    })
    const json = await data.json()
    return json
}

export const Search = () => {
    const [data] = createResource(fetchSearchData)
    
    return (
        <div>
            <div>
                <A href="/">Home</A>
            </div>
            <Show when={!data.loading} fallback={<>Loading...</>}>
                <For each={data()}>
                    {(video, _i) => <VideoCard title={video.title} videoId={video.videoId} />}
                </For>
            </Show>
        </div>
    )
}