import { For } from "solid-js"
import { VideoCard } from "./video-card"

export interface Props {
    data: any
}

export const VideoGrid = (props: Props) => {
    return (
        <div class="grid gap-4 xl:grid-cols-5 lg:grid-cols-4 md:grid-cols-3 grid-cols-2 content-stretch w-full">
            <For each={props.data}>
                {(video, _i) => (
                    <VideoCard 
                        title={video.title} 
                        bannerUrl={video.videoThumbnails?.at(4).url} 
                        videoId={video.videoId} 
                    />
                )}
            </For>
        </div>
    )
}
