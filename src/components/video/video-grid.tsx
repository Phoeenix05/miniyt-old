import { For } from "solid-js"
import { VideoCard } from "./video-card"
import { SearchResult } from "../../util/invidious/search"

export interface Props {
    data: SearchResult[]
}

export const VideoGrid = (props: Props) => {
    return (
        <div class="grid gap-4 xl:grid-cols-5 lg:grid-cols-4 md:grid-cols-3 grid-cols-2 content-stretch w-full mx-2">
            <For each={props.data}>
                {(video, _i) => (
                    <VideoCard data={video} />
                )}
            </For>
        </div>
    )
}
