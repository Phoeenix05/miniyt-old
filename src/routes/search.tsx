import { A, useParams } from "@solidjs/router"
import { Show, createResource, createSignal } from "solid-js"
import { VideoGrid } from "../components/video/video-grid"
import { construct_url } from "../util/url"

export const Search = () => {
    const params = useParams()
    const [page, setPage] = createSignal(1)
    const [videos, setVideos] = createSignal([])
    
    const fetchSearchData = async () => {
        const data = await fetch(construct_url("search", `q=${params.q}&page=${page()}&sort_by=relevance&date=&duration=&type=all`), {
            cache: "default",
        })
        const json: [] = await data.json()
        setVideos([...videos(), ...json.filter(v => v.type == "video")])
    }
    
    const [data] = createResource(page, fetchSearchData)
    
    return (
        <div>
            <div>
                <A href="/">Home</A>
            </div>
            <Show when={videos().length} fallback={<>Loading...</>}>
                <VideoGrid data={videos()} />
            </Show>
            <button onClick={() => setPage(page() + 1)}>Load more...</button>
        </div>
    )
}