import { A, useParams } from "@solidjs/router"
import { Show, createResource, createSignal } from "solid-js"
import { VideoGrid } from "../components/video/video-grid"
import { SearchResult } from "../util/invidious/search"
import { construct_url } from "../util/url"

export const Search = () => {
    const params = useParams()
    const [page, setPage] = createSignal(1)
    const [videos, setVideos] = createSignal<SearchResult[]>([])
    
    const fetchSearchData = async () => {
        const query = `q=${params.q}&page=${page()}&sort_by=relevance&date=&duration=&type=all`
        const json: SearchResult[] = await fetch(construct_url("search", query), { cache: "force-cache" })
            .then(res => res.json())
        setVideos([...videos(), ...json])
    }
    
    const _ = createResource(page, fetchSearchData)
    
    return (
        <>
            <div>
                <A href="/">Home</A>
            </div>
            <Show when={videos().length} fallback={<>Loading...</>}>
                <VideoGrid data={videos()} />
            </Show>
            <button onClick={() => setPage(page() + 1)}>Load more...</button>
        </>
    )
}